use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Deserialize)]
pub struct AssignPermissionDto {
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub navigation_item_id: Uuid,
    pub can_create: bool,
    pub can_read: bool,
    pub can_update: bool,
    pub can_delete: bool,
}

#[derive(Debug, Serialize)]
pub struct PermissionResponseDto {
    pub id: Uuid,
    pub department_id: Option<Uuid>,
    pub department_name: Option<String>,
    pub position_id: Option<Uuid>,
    pub position_name: Option<String>,
    pub navigation_item_id: Uuid,
    pub navigation_name: String,
    pub navigation_path: String,
    pub can_create: bool,
    pub can_read: bool,
    pub can_update: bool,
    pub can_delete: bool,
    pub created_at: NaiveDateTime,
}
