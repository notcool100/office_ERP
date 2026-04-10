use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct DailyLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub log_date: NaiveDate,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct DailyLogWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub log_date: NaiveDate,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct DailyLogLink {
    pub id: Uuid,
    pub daily_log_id: Uuid,
    pub card_id: Uuid,
    pub created_at: NaiveDateTime,
}
