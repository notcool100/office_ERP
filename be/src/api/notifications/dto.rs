use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// What a notification is about. Kept as a plain string in the column so
/// adding a new kind never needs a migration; the mobile app maps unknown
/// kinds to a generic bell icon.
pub mod kind {
    pub const MESSAGE: &str = "message";
    pub const MEETING: &str = "meeting";
    pub const LEAVE: &str = "leave";
    pub const CALENDAR: &str = "calendar";
    pub const DOCUMENT: &str = "document";
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct NotificationResponse {
    pub id: Uuid,
    pub kind: String,
    pub title: String,
    pub body: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationListResponse {
    pub notifications: Vec<NotificationResponse>,
    pub unread_count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListNotificationsQuery {
    pub limit: Option<i64>,
    /// Cursor for paging: return only notifications older than this.
    pub before: Option<DateTime<Utc>>,
    #[serde(default)]
    pub unread_only: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterDeviceRequest {
    pub token: String,
    pub platform: Option<String>,
    pub device_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnregisterDeviceRequest {
    pub token: String,
}

/// A notification about to be created. Built by the feature that raises it
/// (leave approval, new message, ...) and handed to
/// [`crate::api::notifications::service::notify`].
pub struct NewNotification {
    pub kind: &'static str,
    pub title: String,
    pub body: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
}

impl NewNotification {
    pub fn new(kind: &'static str, title: impl Into<String>) -> Self {
        Self {
            kind,
            title: title.into(),
            body: None,
            entity_type: None,
            entity_id: None,
        }
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn entity(mut self, entity_type: &str, entity_id: Uuid) -> Self {
        self.entity_type = Some(entity_type.to_string());
        self.entity_id = Some(entity_id);
        self
    }
}
