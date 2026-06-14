use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ContentCalendarItem {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub content_type: String,
    pub channel: String,
    pub scheduled_date: NaiveDate,
    pub scheduled_time: Option<NaiveTime>,
    pub status: String,
    pub campaign_name: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub created_by: Uuid,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
