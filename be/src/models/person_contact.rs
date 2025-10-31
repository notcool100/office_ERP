use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PersonContact {
    pub id: Uuid,
    pub person_id: Uuid,
    pub email: String,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
}
