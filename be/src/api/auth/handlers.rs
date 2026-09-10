use crate::{
    api::auth::{
        dto::{
            AuthResponse, ChangePasswordRequest, ForgotPasswordRequest, LoginRequest,
            RefreshRequest, RegisterRequest,
        },
        service,
    },
    db::Db,
    models::user::User,
};
use axum::http::StatusCode;
use axum::{Json, extract::Extension};
use serde_json::{Value, json};

/// `(status, body)` for a failure — every auth handler used to return a
/// bare `StatusCode` on error, which axum serializes as an empty body.
/// That's indistinguishable, from a client's point of view, from the
/// request never reaching the server at all: no way to tell "wrong
/// password" apart from "no network", so a login failure had no way to
/// show the user anything more specific than a generic connection error.
type ErrorResponse = (StatusCode, Json<Value>);

fn error_response(status: StatusCode, message: &str) -> ErrorResponse {
    (status, Json(json!({ "message": message })))
}

pub async fn register_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), ErrorResponse> {
    let tokens = service::register(&db, payload).await.map_err(|e| {
        tracing::warn!("register failed: {}", e);
        error_response(
            StatusCode::BAD_REQUEST,
            "Could not create that account. The username or email may already be in use.",
        )
    })?;
    Ok((StatusCode::CREATED, Json(tokens)))
}

pub async fn login_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), ErrorResponse> {
    let tokens = service::login(&db, payload).await.map_err(|e| {
        // Logged at debug, not warn: a mistyped password is routine user
        // error, not something worth flagging in production logs.
        tracing::debug!("login failed: {}", e);
        error_response(StatusCode::UNAUTHORIZED, "Invalid username or password.")
    })?;
    Ok((StatusCode::OK, Json(tokens)))
}

pub async fn refresh_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<RefreshRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), ErrorResponse> {
    let tokens = service::refresh(&db, payload).await.map_err(|e| {
        tracing::debug!("refresh failed: {}", e);
        error_response(
            StatusCode::UNAUTHORIZED,
            "Your session has expired. Please log in again.",
        )
    })?;
    Ok((StatusCode::OK, Json(tokens)))
}

pub async fn forgot_password_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<(StatusCode, Json<Value>), ErrorResponse> {
    service::forgot_password(&db, payload).await.map_err(|e| {
        tracing::error!("forgot_password failed: {}", e);
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Could not send the reset email right now. Please try again shortly.",
        )
    })?;
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Password reset link sent"})),
    ))
}

pub async fn change_password_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<(StatusCode, Json<Value>), ErrorResponse> {
    service::change_password(&db, user.id, payload)
        .await
        .map_err(|e| {
            tracing::warn!("change_password failed for user {}: {}", user.id, e);
            error_response(
                StatusCode::BAD_REQUEST,
                "Could not change your password. Check your current password and try again.",
            )
        })?;
    Ok((StatusCode::OK, Json(json!({"message": "Password changed"}))))
}

pub async fn profile_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Result<(StatusCode, Json<User>), ErrorResponse> {
    let profile = service::get_profile(&db, user.id).await.map_err(|e| {
        tracing::error!("profile lookup failed for user {}: {}", user.id, e);
        error_response(StatusCode::INTERNAL_SERVER_ERROR, "Could not load your profile.")
    })?;
    Ok((StatusCode::OK, Json(profile)))
}
