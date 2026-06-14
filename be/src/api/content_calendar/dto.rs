use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateContentItemDto {
    pub title: String,
    pub description: Option<String>,
    pub content_type: String,
    pub channel: String,
    pub scheduled_date: NaiveDate,
    pub scheduled_time: Option<NaiveTime>,
    pub status: Option<String>,
    pub campaign_name: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateContentItemDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub channel: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub scheduled_time: Option<NaiveTime>,
    pub status: Option<String>,
    pub campaign_name: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListContentItemsQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub channel: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ContentItemResponseDto {
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
    pub assigned_to_name: Option<String>,
    pub created_by: Uuid,
    pub created_by_name: Option<String>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
