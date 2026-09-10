use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateMeetingRequest {
    pub title: Option<String>,
    pub channel_id: Option<Uuid>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MeetingResponse {
    pub id: Uuid,
    pub title: String,
    pub channel_id: Option<Uuid>,
    pub host_id: Option<Uuid>,
    pub host_name: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ParticipantResponse {
    pub user_id: Uuid,
    pub display_name: String,
    pub email: String,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}
