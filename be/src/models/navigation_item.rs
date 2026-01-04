use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct NavigationItem {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub icon: Option<String>,
    pub parent_id: Option<Uuid>,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
