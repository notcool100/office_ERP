use chrono::Utc;
use sqlx::PgPool;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use super::dto::{ListAssetsQuery, MediaAssetFile, MediaAssetMeta, UpdateAssetDto};
use crate::models::user::User;

const MAX_ASSET_SIZE: usize = 50 * 1024 * 1024; // 50 MB

const VALID_CATEGORIES: &[&str] = &[
    "logos", "creatives", "brand-assets", "templates",
    "social-media", "videos", "documents", "other",
];

fn upload_dir() -> PathBuf {
    let base = std::env::var("MEDIA_UPLOAD_DIR")
        .unwrap_or_else(|_| "./uploads/media-library".into());
    PathBuf::from(base)
}

#[derive(Debug, thiserror::Error)]
pub enum MediaLibraryError {
    #[error("Asset not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("{0}")]
    BadRequest(String),
    #[error("Payload too large (max 50 MB)")]
    TooLarge,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

const META_SELECT: &str = r#"
    SELECT
        ma.id, ma.name, ma.description, ma.category, ma.tags,
        ma.file_name, ma.content_type, ma.file_size,
        (ma.content_type LIKE 'image/%') AS is_image,
        ma.uploaded_by, u.user_name AS uploaded_by_name,
        ma.created_at, ma.updated_at
    FROM media_assets ma
    LEFT JOIN users u ON u.id = ma.uploaded_by
"#;

pub async fn list_assets(
    pool: &PgPool,
    query: ListAssetsQuery,
) -> Result<Vec<MediaAssetMeta>, MediaLibraryError> {
    let assets = sqlx::query_as::<_, MediaAssetMeta>(&format!(
        "{} WHERE ($1::text IS NULL OR ma.category = $1)
              AND ($2::text IS NULL OR ma.name ILIKE $2 OR ma.description ILIKE $2
                   OR $2 ILIKE ANY(ma.tags))
         ORDER BY ma.created_at DESC",
        META_SELECT
    ))
    .bind(query.category)
    .bind(query.search.map(|s| format!("%{}%", s)))
    .fetch_all(pool)
    .await?;

    Ok(assets)
}

pub async fn get_asset_meta(pool: &PgPool, id: Uuid) -> Result<MediaAssetMeta, MediaLibraryError> {
    sqlx::query_as::<_, MediaAssetMeta>(&format!("{} WHERE ma.id = $1", META_SELECT))
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(MediaLibraryError::NotFound)
}

pub async fn get_asset_file(pool: &PgPool, id: Uuid) -> Result<MediaAssetFile, MediaLibraryError> {
    sqlx::query_as::<_, MediaAssetFile>(
        "SELECT file_name, content_type, file_path FROM media_assets WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(MediaLibraryError::NotFound)
}

pub async fn upload_asset(
    pool: &PgPool,
    user: &User,
    name: String,
    description: Option<String>,
    category: String,
    tags: Vec<String>,
    file_name: String,
    content_type: String,
    file_data: Vec<u8>,
) -> Result<MediaAssetMeta, MediaLibraryError> {
    if file_data.is_empty() {
        return Err(MediaLibraryError::BadRequest("File is empty".into()));
    }
    if file_data.len() > MAX_ASSET_SIZE {
        return Err(MediaLibraryError::TooLarge);
    }

    let category = if category.is_empty() { "other".to_string() } else { category };
    if !VALID_CATEGORIES.contains(&category.as_str()) {
        return Err(MediaLibraryError::BadRequest(format!("Invalid category: {}", category)));
    }

    let name = if name.trim().is_empty() {
        file_name.clone()
    } else {
        name.trim().to_string()
    };

    // Build a safe, unique filename: <uuid>.<ext>
    let ext = Path::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin")
        .to_lowercase();
    let stored_name = format!("{}.{}", Uuid::new_v4(), ext);

    let dir = upload_dir();
    tokio::fs::create_dir_all(&dir).await?;
    let abs_path = dir.join(&stored_name);
    tokio::fs::write(&abs_path, &file_data).await?;

    let file_path = abs_path.to_string_lossy().into_owned();
    let file_size = file_data.len() as i64;
    let now = Utc::now().naive_utc();

    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO media_assets
            (name, description, category, tags, file_name, content_type, file_size, file_path, uploaded_by, created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$10)
        RETURNING id
        "#,
    )
    .bind(&name)
    .bind(description)
    .bind(&category)
    .bind(&tags)
    .bind(&file_name)
    .bind(&content_type)
    .bind(file_size)
    .bind(&file_path)
    .bind(user.id)
    .bind(now)
    .fetch_one(pool)
    .await?;

    get_asset_meta(pool, id).await
}

pub async fn update_asset(
    pool: &PgPool,
    user: &User,
    id: Uuid,
    dto: UpdateAssetDto,
) -> Result<MediaAssetMeta, MediaLibraryError> {
    let existing = get_asset_meta(pool, id).await?;

    if !user.is_admin && existing.uploaded_by != Some(user.id) {
        return Err(MediaLibraryError::Forbidden);
    }

    if let Some(ref cat) = dto.category {
        if !VALID_CATEGORIES.contains(&cat.as_str()) {
            return Err(MediaLibraryError::BadRequest(format!("Invalid category: {}", cat)));
        }
    }

    sqlx::query(
        r#"
        UPDATE media_assets SET
            name        = COALESCE($1, name),
            description = COALESCE($2, description),
            category    = COALESCE($3, category),
            tags        = COALESCE($4, tags),
            updated_at  = $5
        WHERE id = $6
        "#,
    )
    .bind(dto.name.map(|n| n.trim().to_string()))
    .bind(dto.description)
    .bind(dto.category)
    .bind(dto.tags)
    .bind(Utc::now().naive_utc())
    .bind(id)
    .execute(pool)
    .await?;

    get_asset_meta(pool, id).await
}

pub async fn delete_asset(
    pool: &PgPool,
    user: &User,
    id: Uuid,
) -> Result<(), MediaLibraryError> {
    let existing = get_asset_meta(pool, id).await?;

    if !user.is_admin && existing.uploaded_by != Some(user.id) {
        return Err(MediaLibraryError::Forbidden);
    }

    // Fetch path before deleting the row
    let file = get_asset_file(pool, id).await?;

    sqlx::query("DELETE FROM media_assets WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    // Best-effort disk cleanup — don't fail if file is already gone
    let _ = tokio::fs::remove_file(&file.file_path).await;

    Ok(())
}
