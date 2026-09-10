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

#[derive(Debug, Serialize, FromRow)]
pub struct AttachmentResponse {
    pub id: Uuid,
    pub meeting_id: Uuid,
    pub uploaded_by: Option<Uuid>,
    pub uploaded_by_name: Option<String>,
    pub category: String,
    pub file_name: String,
    pub content_type: String,
    pub file_size: i64,
    pub created_at: DateTime<Utc>,
    /// Absolute on-disk path, only ever used internally by the download
    /// handler — never serialized back to clients.
    #[serde(skip_serializing)]
    pub file_path: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MinutesResponse {
    pub meeting_id: Uuid,
    pub content: String,
    pub updated_by: Option<Uuid>,
    pub updated_by_name: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMinutesRequest {
    pub content: String,
}
