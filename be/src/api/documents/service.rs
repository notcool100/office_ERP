use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use super::dto::{
    CreateFolderRequest, DocCategory, DocumentFile, DocumentMeta, FolderListingResponse,
    FolderMeta, UpdateDocumentRequest,
};
use crate::models::user::User;

const MAX_DOC_SIZE: usize = 50 * 1024 * 1024; // 50 MB
const MAX_BREADCRUMB_DEPTH: usize = 50; // guards against a cyclic parent_id, which should never happen

fn upload_dir() -> PathBuf {
    let base =
        std::env::var("DOCUMENTS_UPLOAD_DIR").unwrap_or_else(|_| "./uploads/documents".into());
    PathBuf::from(base)
}

#[derive(Debug, thiserror::Error)]
pub enum DocumentsError {
    #[error("Not found")]
    NotFound,
    #[error("{0}")]
    BadRequest(String),
    #[error("Payload too large (max 50 MB)")]
    TooLarge,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

/// A company-category request carries no owner id; client/employee
/// categories must. Returns the (client_id, employee_id) column pair to
/// bind into every scoped query below.
fn owner_columns(
    category: DocCategory,
    owner_id: Option<Uuid>,
) -> Result<(Option<Uuid>, Option<Uuid>), DocumentsError> {
    if category.requires_owner() && owner_id.is_none() {
        return Err(DocumentsError::BadRequest("ownerId is required".into()));
    }
    Ok(match category {
        DocCategory::Company => (None, None),
        DocCategory::Client => (owner_id, None),
        DocCategory::Employee => (None, owner_id),
    })
}

const FOLDER_SELECT: &str = r#"
    SELECT
        f.id, f.name, f.parent_id,
        f.created_by, u.user_name AS created_by_name,
        f.created_at, f.updated_at
    FROM document_folders f
    LEFT JOIN users u ON u.id = f.created_by
"#;

const DOCUMENT_SELECT: &str = r#"
    SELECT
        d.id, d.folder_id, d.doc_type, d.title, d.description,
        d.file_name, d.content_type, d.file_size,
        (d.content_type LIKE 'image/%') AS is_image,
        d.expiry_date, d.uploaded_by, u.user_name AS uploaded_by_name,
        d.created_at, d.updated_at
    FROM documents d
    LEFT JOIN users u ON u.id = d.uploaded_by
"#;

async fn folder_breadcrumb(
    pool: &PgPool,
    folder_id: Uuid,
) -> Result<Vec<FolderMeta>, DocumentsError> {
    let mut chain = Vec::new();
    let mut current = Some(folder_id);
    let mut depth = 0;

    while let Some(id) = current {
        depth += 1;
        if depth > MAX_BREADCRUMB_DEPTH {
            break;
        }
        let folder = sqlx::query_as::<_, FolderMeta>(&format!("{} WHERE f.id = $1", FOLDER_SELECT))
            .bind(id)
            .fetch_optional(pool)
            .await?;
        match folder {
            Some(f) => {
                current = f.parent_id;
                chain.push(f);
            }
            None => break,
        }
    }

    chain.reverse();
    Ok(chain)
}

/// Validates that `folder_id` (when given) exists and belongs to the same
/// category/owner — otherwise a caller could nest a folder or file under a
/// folder from a different client/employee/category.
async fn check_folder_in_scope(
    pool: &PgPool,
    category: DocCategory,
    client_id: Option<Uuid>,
    employee_id: Option<Uuid>,
    folder_id: Uuid,
) -> Result<(), DocumentsError> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM document_folders
            WHERE id = $1 AND category = $2
              AND client_id IS NOT DISTINCT FROM $3
              AND employee_id IS NOT DISTINCT FROM $4
        )
        "#,
    )
    .bind(folder_id)
    .bind(category.as_str())
    .bind(client_id)
    .bind(employee_id)
    .fetch_one(pool)
    .await?;

    if exists {
        Ok(())
    } else {
        Err(DocumentsError::NotFound)
    }
}

pub async fn list_location(
    pool: &PgPool,
    category: DocCategory,
    owner_id: Option<Uuid>,
    folder_id: Option<Uuid>,
) -> Result<FolderListingResponse, DocumentsError> {
    let (client_id, employee_id) = owner_columns(category, owner_id)?;

    if let Some(fid) = folder_id {
        check_folder_in_scope(pool, category, client_id, employee_id, fid).await?;
    }

    let folders = sqlx::query_as::<_, FolderMeta>(&format!(
        "{} WHERE f.category = $1 AND f.client_id IS NOT DISTINCT FROM $2
              AND f.employee_id IS NOT DISTINCT FROM $3 AND f.parent_id IS NOT DISTINCT FROM $4
         ORDER BY f.name",
        FOLDER_SELECT
    ))
    .bind(category.as_str())
    .bind(client_id)
    .bind(employee_id)
    .bind(folder_id)
    .fetch_all(pool)
    .await?;

    let documents = sqlx::query_as::<_, DocumentMeta>(&format!(
        "{} WHERE d.category = $1 AND d.client_id IS NOT DISTINCT FROM $2
              AND d.employee_id IS NOT DISTINCT FROM $3 AND d.folder_id IS NOT DISTINCT FROM $4
         ORDER BY d.created_at DESC",
        DOCUMENT_SELECT
    ))
    .bind(category.as_str())
    .bind(client_id)
    .bind(employee_id)
    .bind(folder_id)
    .fetch_all(pool)
    .await?;

    let breadcrumb = match folder_id {
        Some(fid) => folder_breadcrumb(pool, fid).await?,
        None => Vec::new(),
    };

    Ok(FolderListingResponse {
        folders,
        documents,
        breadcrumb,
    })
}

pub async fn create_folder(
    pool: &PgPool,
    user: &User,
    category: DocCategory,
    req: CreateFolderRequest,
) -> Result<FolderMeta, DocumentsError> {
    let (client_id, employee_id) = owner_columns(category, req.owner_id)?;

    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(DocumentsError::BadRequest("Folder name is required".into()));
    }

    if let Some(pid) = req.parent_id {
        check_folder_in_scope(pool, category, client_id, employee_id, pid).await?;
    }

    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM document_folders
            WHERE category = $1 AND client_id IS NOT DISTINCT FROM $2
              AND employee_id IS NOT DISTINCT FROM $3 AND parent_id IS NOT DISTINCT FROM $4
              AND lower(name) = lower($5)
        )
        "#,
    )
    .bind(category.as_str())
    .bind(client_id)
    .bind(employee_id)
    .bind(req.parent_id)
    .bind(&name)
    .fetch_one(pool)
    .await?;

    if exists {
        return Err(DocumentsError::BadRequest(
            "A folder with that name already exists here".into(),
        ));
    }

    let now = Utc::now().naive_utc();
    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO document_folders
            (category, client_id, employee_id, parent_id, name, created_by, created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$7)
        RETURNING id
        "#,
    )
    .bind(category.as_str())
    .bind(client_id)
    .bind(employee_id)
    .bind(req.parent_id)
    .bind(&name)
    .bind(user.id)
    .bind(now)
    .fetch_one(pool)
    .await?;

    sqlx::query_as::<_, FolderMeta>(&format!("{} WHERE f.id = $1", FOLDER_SELECT))
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(DocumentsError::from)
}

/// Recursively collects file_path for every document nested (at any depth)
/// under `folder_id`, so they can be removed from disk after the row —
/// and its DB-cascaded descendants — are deleted.
async fn collect_file_paths_under(
    pool: &PgPool,
    folder_id: Uuid,
) -> Result<Vec<String>, DocumentsError> {
    let paths = sqlx::query_scalar::<_, String>(
        r#"
        WITH RECURSIVE subtree AS (
            SELECT id FROM document_folders WHERE id = $1
            UNION ALL
            SELECT f.id FROM document_folders f
            INNER JOIN subtree s ON f.parent_id = s.id
        )
        SELECT file_path FROM documents WHERE folder_id IN (SELECT id FROM subtree)
        "#,
    )
    .bind(folder_id)
    .fetch_all(pool)
    .await?;
    Ok(paths)
}

pub async fn delete_folder(
    pool: &PgPool,
    _user: &User,
    category: DocCategory,
    owner_id: Option<Uuid>,
    folder_id: Uuid,
) -> Result<(), DocumentsError> {
    let (client_id, employee_id) = owner_columns(category, owner_id)?;
    check_folder_in_scope(pool, category, client_id, employee_id, folder_id).await?;

    let file_paths = collect_file_paths_under(pool, folder_id).await?;

    sqlx::query("DELETE FROM document_folders WHERE id = $1")
        .bind(folder_id)
        .execute(pool)
        .await?;

    for path in file_paths {
        let _ = tokio::fs::remove_file(&path).await;
    }

    Ok(())
}

pub async fn get_document_meta(
    pool: &PgPool,
    category: DocCategory,
    owner_id: Option<Uuid>,
    id: Uuid,
) -> Result<DocumentMeta, DocumentsError> {
    let (client_id, employee_id) = owner_columns(category, owner_id)?;
    sqlx::query_as::<_, DocumentMeta>(&format!(
        "{} WHERE d.id = $1 AND d.category = $2 AND d.client_id IS NOT DISTINCT FROM $3
              AND d.employee_id IS NOT DISTINCT FROM $4",
        DOCUMENT_SELECT
    ))
    .bind(id)
    .bind(category.as_str())
    .bind(client_id)
    .bind(employee_id)
    .fetch_optional(pool)
    .await?
    .ok_or(DocumentsError::NotFound)
}

pub async fn get_document_file(
    pool: &PgPool,
    category: DocCategory,
    owner_id: Option<Uuid>,
    id: Uuid,
) -> Result<DocumentFile, DocumentsError> {
    let (client_id, employee_id) = owner_columns(category, owner_id)?;
    sqlx::query_as::<_, DocumentFile>(
        r#"
        SELECT file_name, content_type, file_path FROM documents
        WHERE id = $1 AND category = $2 AND client_id IS NOT DISTINCT FROM $3
          AND employee_id IS NOT DISTINCT FROM $4
        "#,
    )
    .bind(id)
    .bind(category.as_str())
    .bind(client_id)
    .bind(employee_id)
    .fetch_optional(pool)
    .await?
    .ok_or(DocumentsError::NotFound)
}

#[allow(clippy::too_many_arguments)]
pub async fn upload_document(
    pool: &PgPool,
    user: &User,
    category: DocCategory,
    owner_id: Option<Uuid>,
    folder_id: Option<Uuid>,
    title: String,
    doc_type: Option<String>,
    description: Option<String>,
    expiry_date: Option<NaiveDate>,
    file_name: String,
    content_type: String,
    file_data: Vec<u8>,
) -> Result<DocumentMeta, DocumentsError> {
    if file_data.is_empty() {
        return Err(DocumentsError::BadRequest("File is empty".into()));
    }
    if file_data.len() > MAX_DOC_SIZE {
        return Err(DocumentsError::TooLarge);
    }

    let (client_id, employee_id) = owner_columns(category, owner_id)?;

    if let Some(fid) = folder_id {
        check_folder_in_scope(pool, category, client_id, employee_id, fid).await?;
    }

    let title = if title.trim().is_empty() {
        file_name.clone()
    } else {
        title.trim().to_string()
    };

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
        INSERT INTO documents
            (category, client_id, employee_id, folder_id, doc_type, title, description,
             file_name, content_type, file_size, file_path, expiry_date, uploaded_by,
             created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$14)
        RETURNING id
        "#,
    )
    .bind(category.as_str())
    .bind(client_id)
    .bind(employee_id)
    .bind(folder_id)
    .bind(doc_type)
    .bind(&title)
    .bind(description)
    .bind(&file_name)
    .bind(&content_type)
    .bind(file_size)
    .bind(&file_path)
    .bind(expiry_date)
    .bind(user.id)
    .bind(now)
    .fetch_one(pool)
    .await?;

    get_document_meta(pool, category, owner_id, id).await
}

pub async fn update_document(
    pool: &PgPool,
    category: DocCategory,
    owner_id: Option<Uuid>,
    id: Uuid,
    dto: UpdateDocumentRequest,
) -> Result<DocumentMeta, DocumentsError> {
    // Ensures the row is in scope before allowing the update.
    get_document_meta(pool, category, owner_id, id).await?;

    sqlx::query(
        r#"
        UPDATE documents SET
            title       = COALESCE($1, title),
            description = COALESCE($2, description),
            doc_type    = COALESCE($3, doc_type),
            expiry_date = COALESCE($4, expiry_date),
            updated_at  = $5
        WHERE id = $6
        "#,
    )
    .bind(dto.title.map(|t| t.trim().to_string()))
    .bind(dto.description)
    .bind(dto.doc_type)
    .bind(dto.expiry_date)
    .bind(Utc::now().naive_utc())
    .bind(id)
    .execute(pool)
    .await?;

    get_document_meta(pool, category, owner_id, id).await
}

pub async fn delete_document(
    pool: &PgPool,
    category: DocCategory,
    owner_id: Option<Uuid>,
    id: Uuid,
) -> Result<(), DocumentsError> {
    let file = get_document_file(pool, category, owner_id, id).await?;
    let (client_id, employee_id) = owner_columns(category, owner_id)?;

    sqlx::query(
        "DELETE FROM documents WHERE id = $1 AND category = $2
         AND client_id IS NOT DISTINCT FROM $3 AND employee_id IS NOT DISTINCT FROM $4",
    )
    .bind(id)
    .bind(category.as_str())
    .bind(client_id)
    .bind(employee_id)
    .execute(pool)
    .await?;

    let _ = tokio::fs::remove_file(&file.file_path).await;

    Ok(())
}
