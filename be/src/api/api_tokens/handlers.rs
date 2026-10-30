use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::{db::Db, models::user::User};

use super::service;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiTokenDto {
    pub name: String,
    /// Omit for a token that never expires.
    pub expires_in_days: Option<i64>,
}

fn server_error(e: sqlx::Error) -> axum::response::Response {
    tracing::error!("api token error: {:?}", e);
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

pub async fn list_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    match service::list(&db, user.id).await {
        Ok(tokens) => (StatusCode::OK, Json(tokens)).into_response(),
        Err(e) => server_error(e),
    }
}

pub async fn create_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(dto): Json<CreateApiTokenDto>,
) -> impl IntoResponse {
    let name = dto.name.trim();
    if name.is_empty() || name.len() > 100 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Name is required (max 100 characters)" })),
        )
            .into_response();
    }
    if dto
        .expires_in_days
        .is_some_and(|d| !(1..=3650).contains(&d))
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "expiresInDays must be between 1 and 3650" })),
        )
            .into_response();
    }

    match service::create(&db, user.id, name, dto.expires_in_days).await {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(e) => server_error(e),
    }
}

pub async fn revoke_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service::revoke(&db, user.id, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => server_error(e),
    }
}
