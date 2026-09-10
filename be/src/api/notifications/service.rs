use anyhow::Result;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    db::Db,
    push::{self, PushMessage, SendOutcome},
    ws::hub::{Hub, WsMessage},
};

use super::dto::{
    ListNotificationsQuery, NewNotification, NotificationListResponse, NotificationResponse,
    RegisterDeviceRequest,
};

/// The read cap on one page of the inbox. The app pages with `before`.
const DEFAULT_LIMIT: i64 = 50;
const MAX_LIMIT: i64 = 100;

/// Records a notification, pushes it down the recipient's WebSocket, and
/// fans it out to their registered devices.
///
/// The database write is awaited (the inbox must be correct); delivery is
/// spawned, because the caller is usually mid-request handling something
/// else — approving leave, sending a message — and that request must not
/// block on, or fail because of, Google's servers.
pub async fn notify(
    db: &Db,
    hub: Option<&Arc<Hub>>,
    user_id: Uuid,
    notification: NewNotification,
) -> Result<NotificationResponse> {
    let stored = sqlx::query_as::<_, NotificationResponse>(
        r#"
        INSERT INTO notifications (user_id, kind, title, body, entity_type, entity_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, kind, title, body, entity_type, entity_id, read_at, created_at
        "#,
    )
    .bind(user_id)
    .bind(notification.kind)
    .bind(&notification.title)
    .bind(&notification.body)
    .bind(&notification.entity_type)
    .bind(notification.entity_id)
    .fetch_one(db)
    .await?;

    if let Some(hub) = hub {
        hub.send_to_user(
            user_id,
            WsMessage {
                message_type: "notification".to_string(),
                payload: json!(stored),
            },
        );
    }

    if push::is_enabled() {
        let db = db.clone();
        let payload = stored.clone();
        tokio::spawn(async move {
            if let Err(e) = push_to_devices(&db, user_id, &payload).await {
                tracing::warn!("push fan-out for user {} failed: {}", user_id, e);
            }
        });
    }

    Ok(stored)
}

/// Same as [`notify`] but for a set of recipients, skipping `exclude`
/// (normally the person who caused the event — nobody wants a push for
/// their own message).
pub async fn notify_many(
    db: &Db,
    hub: Option<&Arc<Hub>>,
    user_ids: &[Uuid],
    exclude: Option<Uuid>,
    build: impl Fn() -> NewNotification,
) {
    for user_id in user_ids {
        if Some(*user_id) == exclude {
            continue;
        }
        if let Err(e) = notify(db, hub, *user_id, build()).await {
            tracing::warn!("could not notify user {}: {}", user_id, e);
        }
    }
}

async fn push_to_devices(db: &Db, user_id: Uuid, payload: &NotificationResponse) -> Result<()> {
    let tokens = sqlx::query_scalar::<_, String>(
        "SELECT token FROM device_tokens WHERE user_id = $1 ORDER BY last_seen_at DESC",
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    if tokens.is_empty() {
        return Ok(());
    }

    let message = PushMessage {
        title: payload.title.clone(),
        body: payload.body.clone().unwrap_or_default(),
        data: json!({
            "notificationId": payload.id,
            "kind": payload.kind,
            "entityType": payload.entity_type,
            "entityId": payload.entity_id,
        }),
    };

    for token in tokens {
        if let SendOutcome::TokenInvalid = push::send_to_token(&token, &message).await {
            let _ = sqlx::query("DELETE FROM device_tokens WHERE token = $1")
                .bind(&token)
                .execute(db)
                .await;
        }
    }

    Ok(())
}

pub async fn list(
    db: &Db,
    user_id: Uuid,
    query: ListNotificationsQuery,
) -> Result<NotificationListResponse> {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);

    let notifications = sqlx::query_as::<_, NotificationResponse>(
        r#"
        SELECT id, kind, title, body, entity_type, entity_id, read_at, created_at
        FROM notifications
        WHERE user_id = $1
          AND ($2::timestamptz IS NULL OR created_at < $2)
          AND ($3 = false OR read_at IS NULL)
        ORDER BY created_at DESC
        LIMIT $4
        "#,
    )
    .bind(user_id)
    .bind(query.before)
    .bind(query.unread_only)
    .bind(limit)
    .fetch_all(db)
    .await?;

    Ok(NotificationListResponse {
        notifications,
        unread_count: unread_count(db, user_id).await?,
    })
}

pub async fn unread_count(db: &Db, user_id: Uuid) -> Result<i64> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM notifications WHERE user_id = $1 AND read_at IS NULL",
    )
    .bind(user_id)
    .fetch_one(db)
    .await?;
    Ok(count)
}

pub async fn mark_read(db: &Db, user_id: Uuid, id: Uuid) -> Result<()> {
    sqlx::query(
        "UPDATE notifications SET read_at = NOW() \
         WHERE id = $1 AND user_id = $2 AND read_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn mark_all_read(db: &Db, user_id: Uuid) -> Result<()> {
    sqlx::query("UPDATE notifications SET read_at = NOW() WHERE user_id = $1 AND read_at IS NULL")
        .bind(user_id)
        .execute(db)
        .await?;
    Ok(())
}

/// Upserts on the token, not on (user, token): a registration token
/// identifies a device install, and if the same install signs in as
/// someone else it must stop receiving the previous user's pushes.
pub async fn register_device(db: &Db, user_id: Uuid, req: RegisterDeviceRequest) -> Result<()> {
    let platform = req.platform.unwrap_or_else(|| "android".to_string());

    sqlx::query(
        r#"
        INSERT INTO device_tokens (user_id, token, platform, device_name)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (token) DO UPDATE
        SET user_id = EXCLUDED.user_id,
            platform = EXCLUDED.platform,
            device_name = EXCLUDED.device_name,
            last_seen_at = NOW()
        "#,
    )
    .bind(user_id)
    .bind(&req.token)
    .bind(&platform)
    .bind(&req.device_name)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn unregister_device(db: &Db, user_id: Uuid, token: &str) -> Result<()> {
    sqlx::query("DELETE FROM device_tokens WHERE user_id = $1 AND token = $2")
        .bind(user_id)
        .bind(token)
        .execute(db)
        .await?;
    Ok(())
}
