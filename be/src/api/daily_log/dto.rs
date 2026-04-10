use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDailyLogDto {
    pub log_date: NaiveDate,
    pub content: String,
    pub card_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDailyLogDto {
    pub log_date: Option<NaiveDate>,
    pub content: Option<String>,
    pub card_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyLogResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub log_date: NaiveDate,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub links: Vec<DailyLogLinkResponse>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DailyLogLinkResponse {
    pub id: Uuid,
    pub card_id: Uuid,
    pub card_key: String,
    pub card_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListDailyLogQuery {
    pub user_id: Option<Uuid>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}
