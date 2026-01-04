use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct RolePermission {
    pub id: Uuid,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub navigation_item_id: Uuid,
    pub can_create: bool,
    pub can_read: bool,
    pub can_update: bool,
    pub can_delete: bool,
    pub created_at: NaiveDateTime,
}
