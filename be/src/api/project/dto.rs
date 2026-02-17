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
    pub priority: Option<String>,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCardDto {
    pub column_id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CardResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub column_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub assignee_id: Option<Uuid>,
    pub assignee_name: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub display_order: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ListCardsQuery {
    pub column_id: Option<Uuid>,
}
