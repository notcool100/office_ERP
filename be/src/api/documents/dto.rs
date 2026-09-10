use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Which of the three document surfaces a request is scoped to. Bound to
/// the router as an `Extension` at mount time (see routes.rs) rather than
/// trusted from client input, so a caller granted access to one surface
/// (e.g. Client Documents) can't address rows in another (e.g. Employee
/// Documents) by passing a different category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocCategory {
    Company,
    Client,
    Employee,
}

impl DocCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            DocCategory::Company => "company",
            DocCategory::Client => "client",
            DocCategory::Employee => "employee",
        }
    }

    /// Whether this category requires an owner id (client_id / employee_id).
    pub fn requires_owner(self) -> bool {
        !matches!(self, DocCategory::Company)
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct FolderMeta {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_by_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DocumentMeta {
    pub id: Uuid,
    pub folder_id: Option<Uuid>,
    pub doc_type: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub file_name: String,
    pub content_type: String,
    pub file_size: i64,
    pub is_image: bool,
    pub expiry_date: Option<NaiveDate>,
    pub uploaded_by: Option<Uuid>,
    pub uploaded_by_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Used internally for file serving.
#[derive(Debug, sqlx::FromRow)]
pub struct DocumentFile {
    pub file_name: String,
    pub content_type: String,
    pub file_path: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderListingResponse {
    pub folders: Vec<FolderMeta>,
    pub documents: Vec<DocumentMeta>,
    pub breadcrumb: Vec<FolderMeta>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLocationQuery {
    pub owner_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerQuery {
    pub owner_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub owner_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocumentRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub doc_type: Option<String>,
    pub expiry_date: Option<NaiveDate>,
}
