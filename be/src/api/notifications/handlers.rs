use axum::{
    Json,
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::{db::Db, models::user::User};

use super::{
    dto::{ListNotificationsQuery, RegisterDeviceRequest, UnregisterDeviceRequest},
    service,
};

fn server_error(e: anyhow::Error) -> axum::response::Response {
    tracing::error!("notifications: {}", e);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "message": "Something went wrong" })),
    )
        .into_response()
}

pub async fn list_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Query(query): Query<ListNotificationsQuery>,
) -> impl IntoResponse {
    match service::list(&db, user.id, query).await {
        Ok(response) => (StatusCode::OK, Json(json!(response))).into_response(),
        Err(e) => server_error(e),
    }
}

pub async fn unread_count_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    match service::unread_count(&db, user.id).await {
        Ok(count) => (StatusCode::OK, Json(json!({ "unreadCount": count }))).into_response(),
        Err(e) => server_error(e),
    }
}

pub async fn mark_read_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service::mark_read(&db, user.id, id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => server_error(e),
    }
}

pub async fn mark_all_read_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    match service::mark_all_read(&db, user.id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => server_error(e),
    }
}

pub async fn register_device_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<RegisterDeviceRequest>,
) -> impl IntoResponse {
    if payload.token.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "token is required" })),
        )
            .into_response();
    }

    match service::register_device(&db, user.id, payload).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => server_error(e),
    }
}

pub async fn unregister_device_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<UnregisterDeviceRequest>,
) -> impl IntoResponse {
    match service::unregister_device(&db, user.id, &payload.token).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => server_error(e),
    }
}
