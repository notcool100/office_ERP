use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Deserialize)]
pub struct CreateNavigationItemDto {
    pub name: String,
    pub path: String,
    pub icon: Option<String>,
    pub parent_id: Option<Uuid>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNavigationItemDto {
    pub name: Option<String>,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<Uuid>,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct NavigationItemResponseDto {
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

#[derive(Debug, Serialize)]
pub struct UserNavigationItemDto {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub icon: Option<String>,
    pub parent_id: Option<Uuid>,
    pub display_order: i32,
    pub can_create: bool,
    pub can_read: bool,
    pub can_update: bool,
    pub can_delete: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<UserNavigationItemDto>,
}
