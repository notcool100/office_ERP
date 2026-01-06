use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    #[serde(skip)]
    pub password_hash: String,
    pub email: String,
    pub phone: String,
    pub person_id: Uuid,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
}
