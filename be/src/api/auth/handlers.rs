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
use serde_json::json;

pub async fn register_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), StatusCode> {
    let tokens = service::register(&db, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(tokens)))
}

pub async fn login_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), StatusCode> {
    let tokens = service::login(&db, payload)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok((StatusCode::OK, Json(tokens)))
}

pub async fn refresh_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<RefreshRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), StatusCode> {
    let tokens = service::refresh(&db, payload)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    Ok((StatusCode::OK, Json(tokens)))
}

pub async fn forgot_password_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::forgot_password(&db, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Password reset link sent"})),
    ))
}

pub async fn change_password_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::change_password(&db, user.id, payload)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(json!({"message": "Password changed"}))))
}

pub async fn profile_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let profile = service::get_profile(&db, user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(profile)))
}
