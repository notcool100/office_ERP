use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::db::Db;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NotificationSetting {
    pub key: String,
    pub label: String,
    pub description: Option<String>,
    pub emails: String, // JSON array stored as text
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationSettingResponse {
    pub key: String,
    pub label: String,
    pub description: Option<String>,
    pub emails: Vec<String>,
}

impl From<NotificationSetting> for NotificationSettingResponse {
    fn from(s: NotificationSetting) -> Self {
        let emails: Vec<String> =
            serde_json::from_str(&s.emails).unwrap_or_default();
        Self {
            key: s.key,
            label: s.label,
            description: s.description,
            emails,
        }
    }
}

pub async fn list(pool: &Db) -> Result<Vec<NotificationSettingResponse>> {
    let rows = sqlx::query_as::<_, NotificationSetting>(
        "SELECT key, label, description, emails FROM notification_settings ORDER BY key",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn get(pool: &Db, key: &str) -> Result<NotificationSettingResponse> {
    let row = sqlx::query_as::<_, NotificationSetting>(
        "SELECT key, label, description, emails FROM notification_settings WHERE key = $1",
    )
    .bind(key)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Notification setting '{}' not found", key))?;

    Ok(row.into())
}

pub async fn update_emails(
    pool: &Db,
    key: &str,
    emails: Vec<String>,
) -> Result<NotificationSettingResponse> {
    // Deduplicate and lowercase
    let mut deduped: Vec<String> = emails
        .into_iter()
        .map(|e| e.trim().to_lowercase())
        .filter(|e| !e.is_empty() && e.contains('@'))
        .collect();
    deduped.sort();
    deduped.dedup();

    let emails_json = serde_json::to_string(&deduped)?;

    let row = sqlx::query_as::<_, NotificationSetting>(
        r#"
        UPDATE notification_settings
        SET emails = $1, updated_at = NOW()
        WHERE key = $2
        RETURNING key, label, description, emails
        "#,
    )
    .bind(&emails_json)
    .bind(key)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("Notification setting '{}' not found", key))?;

    Ok(row.into())
}

/// Fetch the email list for a given key — used internally by other services.
pub async fn get_emails(pool: &Db, key: &str) -> Vec<String> {
    let row = sqlx::query_scalar::<_, String>(
        "SELECT emails FROM notification_settings WHERE key = $1",
    )
    .bind(key)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten();

    row.and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}
