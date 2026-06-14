use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Returned in list and single-get responses (no file_data).
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MediaAssetMeta {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub tags: Vec<String>,
    pub file_name: String,
    pub content_type: String,
    pub file_size: i64,
    pub is_image: bool,
    pub uploaded_by: Option<Uuid>,
    pub uploaded_by_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Used internally for file serving.
#[derive(Debug, sqlx::FromRow)]
pub struct MediaAssetFile {
    pub file_name: String,
    pub content_type: String,
    pub file_path: String,
}

/// Query params for listing.
#[derive(Debug, Deserialize)]
pub struct ListAssetsQuery {
    pub category: Option<String>,
    pub search: Option<String>,
}

/// JSON body for metadata-only updates.
#[derive(Debug, Deserialize)]
pub struct UpdateAssetDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}
