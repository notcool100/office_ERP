use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Card {
    pub id: Uuid,
    pub project_id: Uuid,
    pub column_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub assignee_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
    pub display_order: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
