use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateCalendarEventDto {
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub all_day: Option<bool>,
    pub scope: Option<String>,
    pub department_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCalendarEventDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_at: Option<NaiveDateTime>,
    pub end_at: Option<NaiveDateTime>,
    pub all_day: Option<bool>,
    pub scope: Option<String>,
    pub department_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct CalendarEventResponseDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_at: NaiveDateTime,
    pub end_at: NaiveDateTime,
    pub all_day: bool,
    pub scope: String,
    pub department_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ListCalendarEventsQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}
