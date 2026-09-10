use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    /// For a DM (private, 2-member) channel: the display name of the other
    /// participant, resolved per-viewer so each side sees the other
    /// person's name rather than a name baked into `description` at
    /// creation time. Null for non-DM channels, and also right after
    /// `INSERT ... RETURNING *` where the query doesn't select it.
    #[sqlx(default)]
    pub dm_other_user_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChannelMember {
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}
