use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub members: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateChannelRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RemoveMemberRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MessageResponse {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub sender_name: Option<String>,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Populated separately from `message_attachments` after the base
    /// message row is fetched — never a real column in the queries below,
    /// so `#[sqlx(skip)]` is required (the type has no Postgres decode
    /// impl and is never meant to be read from a row).
    #[sqlx(skip)]
    pub attachments: Vec<AttachmentResponse>,
    /// Same story as `attachments`: grouped from `message_reactions` after
    /// the base row is fetched, viewer-dependent (`reacted_by_me`), so it
    /// can never be part of the SELECT itself.
    #[sqlx(skip)]
    pub reactions: Vec<ReactionSummary>,
}

/// One emoji's aggregate on a message: how many people reacted with it,
/// and whether the requesting user is one of them.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ReactionSummary {
    pub emoji: String,
    pub count: i64,
    pub reacted_by_me: bool,
}

#[derive(Debug, Deserialize)]
pub struct ToggleReactionRequest {
    pub emoji: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct AttachmentResponse {
    pub id: Uuid,
    pub message_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub file_size: i64,
    pub is_image: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ChannelMediaItem {
    pub id: Uuid,
    pub message_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub file_size: i64,
    pub is_image: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub sender_id: Option<Uuid>,
    pub sender_name: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ChannelMemberResponse {
    pub id: Uuid,
    pub display_name: String,
    pub email: String,
}
