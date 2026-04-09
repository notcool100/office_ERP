use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateProjectDto {
    pub project_key: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponseDto {
    pub id: Uuid,
    pub project_key: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub member_role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddProjectMemberDto {
    pub user_id: Uuid,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectMemberResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub email: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct BoardColumnResponseDto {
    pub id: Uuid,
    pub board_id: Uuid,
    pub name: String,
    pub is_done: bool,
    pub display_order: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct BoardResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub columns: Vec<BoardColumnResponseDto>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCardDto {
    pub column_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub sprint_name: Option<String>,
    pub card_type: Option<String>,
    pub parent_id: Option<Uuid>,
    pub priority: Option<String>,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
    pub display_order: Option<i32>,
    pub sprint_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCardDto {
    pub column_id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub sprint_name: Option<String>,
    pub card_type: Option<String>,
    pub parent_id: Option<Uuid>,
    pub priority: Option<String>,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
    pub display_order: Option<i32>,
    pub sprint_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct CardResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub column_id: Option<Uuid>,
    pub sequence_no: i32,
    pub card_key: String,
    pub title: String,
    pub description: Option<String>,
    pub card_type: String,
    pub parent_id: Option<Uuid>,
    pub parent_card_key: Option<String>,
    pub is_migrated: bool,
    pub sprint_name: Option<String>,
    pub priority: String,
    pub assignee_id: Option<Uuid>,
    pub assignee_name: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub display_order: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub sprint_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ListCardsQuery {
    pub column_id: Option<Uuid>,
    pub sprint_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSprintDto {
    pub name: String,
    pub goal: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSprintDto {
    pub name: Option<String>,
    pub goal: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SprintResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub goal: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCardCommentDto {
    pub comment: String,
}

#[derive(Debug, Serialize)]
pub struct CardCommentResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub card_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub comment: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct CardAttachmentResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub card_id: Uuid,
    pub uploaded_by: Option<Uuid>,
    pub uploader_name: Option<String>,
    pub file_name: String,
    pub content_type: String,
    pub file_size: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct CardActivityResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub card_id: Uuid,
    pub actor_id: Option<Uuid>,
    pub actor_name: Option<String>,
    pub action_type: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCardLinkDto {
    pub target_card_id: Uuid,
    pub link_type: String,
}

#[derive(Debug, Serialize)]
pub struct CardLinkResponseDto {
    pub id: Uuid,
    pub source_card_id: Uuid,
    pub target_card_id: Uuid,
    pub source_card_key: String,
    pub target_card_key: String,
    pub source_title: String,
    pub target_title: String,
    pub link_type: String,
    pub created_at: NaiveDateTime,
}
