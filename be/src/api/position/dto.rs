use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreatePositionDto {
    pub name: String,
    pub description: Option<String>,
    pub department_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePositionDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub department_id: Option<Uuid>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PositionResponseDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub department_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
