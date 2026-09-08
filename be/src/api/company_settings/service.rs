use chrono::{NaiveDateTime, Utc};
use sqlx::PgPool;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const MAX_IMAGE_SIZE: usize = 5 * 1024 * 1024; // 5 MB
pub const DEFAULT_COMPANY_NAME: &str = "Adya Technologies";

const DEFAULT_LOGO_BYTES: &[u8] = include_bytes!("../../../assets/default-logo.jpeg");
const DEFAULT_FAVICON_BYTES: &[u8] = include_bytes!("../../../assets/default-favicon.png");

#[derive(Debug, thiserror::Error)]
pub enum CompanySettingsError {
    #[error("Not found")]
    NotFound,
    #[error("{0}")]
    BadRequest(String),
    #[error("Image is too large (max 5 MB)")]
    TooLarge,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

#[derive(sqlx::FromRow)]
pub struct CompanySettingsRow {
    pub id: Uuid,
    pub name: String,
    pub logo_path: Option<String>,
    pub logo_content_type: Option<String>,
    pub favicon_path: Option<String>,
    pub favicon_content_type: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub location_name: Option<String>,
    pub updated_at: NaiveDateTime,
}

pub struct StoredFile {
    pub file_path: String,
    pub content_type: String,
}

fn upload_dir() -> PathBuf {
    let base =
        std::env::var("COMPANY_UPLOAD_DIR").unwrap_or_else(|_| "./uploads/company".to_string());
    PathBuf::from(base)
}

async fn get_row(pool: &PgPool) -> Result<CompanySettingsRow, CompanySettingsError> {
    sqlx::query_as::<_, CompanySettingsRow>(
        "SELECT id, name, logo_path, logo_content_type, favicon_path, favicon_content_type, \
         latitude, longitude, location_name, updated_at \
         FROM company_settings ORDER BY updated_at DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?
    .ok_or(CompanySettingsError::NotFound)
}

pub async fn get_settings(pool: &PgPool) -> Result<CompanySettingsRow, CompanySettingsError> {
    get_row(pool).await
}

/// Best-effort company name for use in outgoing emails - never fails, so a
/// company_settings hiccup never blocks an unrelated email from sending.
pub async fn get_company_name(pool: &PgPool) -> String {
    match get_row(pool).await {
        Ok(row) => row.name,
        Err(_) => DEFAULT_COMPANY_NAME.to_string(),
    }
}

/// Fills in the bundled default logo/favicon on first boot (or whenever
/// either is unset), so company_settings always has real images behind it
/// instead of a null/placeholder state.
pub async fn ensure_default_branding(pool: &PgPool) -> Result<(), CompanySettingsError> {
    let row = get_row(pool).await?;

    if row.logo_path.is_none() {
        let path = store_image("default-logo.jpeg", DEFAULT_LOGO_BYTES).await?;
        sqlx::query(
            "UPDATE company_settings SET logo_path = $1, logo_content_type = $2 WHERE id = $3",
        )
        .bind(&path)
        .bind("image/jpeg")
        .bind(row.id)
        .execute(pool)
        .await?;
    }

    if row.favicon_path.is_none() {
        let path = store_image("default-favicon.png", DEFAULT_FAVICON_BYTES).await?;
        sqlx::query(
            "UPDATE company_settings SET favicon_path = $1, favicon_content_type = $2 WHERE id = $3",
        )
        .bind(&path)
        .bind("image/png")
        .bind(row.id)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn update_details(
    pool: &PgPool,
    name: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
    location_name: Option<String>,
) -> Result<CompanySettingsRow, CompanySettingsError> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err(CompanySettingsError::BadRequest(
            "Company name is required".to_string(),
        ));
    }

    let row = get_row(pool).await?;
    let now = Utc::now().naive_utc();

    sqlx::query(
        r#"
        UPDATE company_settings SET
            name = $1,
            latitude = $2,
            longitude = $3,
            location_name = $4,
            updated_at = $5
        WHERE id = $6
        "#,
    )
    .bind(&name)
    .bind(latitude)
    .bind(longitude)
    .bind(&location_name)
    .bind(now)
    .bind(row.id)
    .execute(pool)
    .await?;

    get_row(pool).await
}

fn validate_image(content_type: &str, data: &[u8]) -> Result<(), CompanySettingsError> {
    if data.is_empty() {
        return Err(CompanySettingsError::BadRequest(
            "File is empty".to_string(),
        ));
    }
    if data.len() > MAX_IMAGE_SIZE {
        return Err(CompanySettingsError::TooLarge);
    }
    if !content_type.starts_with("image/") {
        return Err(CompanySettingsError::BadRequest(
            "File must be an image".to_string(),
        ));
    }
    Ok(())
}

async fn store_image(file_name: &str, data: &[u8]) -> Result<String, CompanySettingsError> {
    let ext = Path::new(file_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin")
        .to_lowercase();
    let stored_name = format!("{}.{}", Uuid::new_v4(), ext);

    let dir = upload_dir();
    tokio::fs::create_dir_all(&dir).await?;
    let abs_path = dir.join(&stored_name);
    tokio::fs::write(&abs_path, data).await?;

    Ok(abs_path.to_string_lossy().into_owned())
}

pub async fn update_logo(
    pool: &PgPool,
    file_name: &str,
    content_type: &str,
    data: &[u8],
) -> Result<CompanySettingsRow, CompanySettingsError> {
    validate_image(content_type, data)?;
    let row = get_row(pool).await?;
    let new_path = store_image(file_name, data).await?;

    sqlx::query(
        "UPDATE company_settings SET logo_path = $1, logo_content_type = $2, updated_at = $3 WHERE id = $4",
    )
    .bind(&new_path)
    .bind(content_type)
    .bind(Utc::now().naive_utc())
    .bind(row.id)
    .execute(pool)
    .await?;

    if let Some(old_path) = row.logo_path {
        let _ = tokio::fs::remove_file(&old_path).await;
    }

    get_row(pool).await
}

pub async fn update_favicon(
    pool: &PgPool,
    file_name: &str,
    content_type: &str,
    data: &[u8],
) -> Result<CompanySettingsRow, CompanySettingsError> {
    validate_image(content_type, data)?;
    let row = get_row(pool).await?;
    let new_path = store_image(file_name, data).await?;

    sqlx::query(
        "UPDATE company_settings SET favicon_path = $1, favicon_content_type = $2, updated_at = $3 WHERE id = $4",
    )
    .bind(&new_path)
    .bind(content_type)
    .bind(Utc::now().naive_utc())
    .bind(row.id)
    .execute(pool)
    .await?;

    if let Some(old_path) = row.favicon_path {
        let _ = tokio::fs::remove_file(&old_path).await;
    }

    get_row(pool).await
}

pub async fn get_logo_file(pool: &PgPool) -> Result<StoredFile, CompanySettingsError> {
    let row = get_row(pool).await?;
    match (row.logo_path, row.logo_content_type) {
        (Some(file_path), Some(content_type)) => Ok(StoredFile {
            file_path,
            content_type,
        }),
        _ => Err(CompanySettingsError::NotFound),
    }
}

pub async fn get_favicon_file(pool: &PgPool) -> Result<StoredFile, CompanySettingsError> {
    let row = get_row(pool).await?;
    match (row.favicon_path, row.favicon_content_type) {
        (Some(file_path), Some(content_type)) => Ok(StoredFile {
            file_path,
            content_type,
        }),
        _ => Err(CompanySettingsError::NotFound),
    }
}
