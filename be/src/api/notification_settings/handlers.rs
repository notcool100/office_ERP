use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::db::Db;

use super::service;

#[derive(Deserialize)]
pub struct UpdateEmailsDto {
    pub emails: Vec<String>,
}

pub async fn list_handler(Extension(db): Extension<Db>) -> impl IntoResponse {
    match service::list(&db).await {
        Ok(settings) => (StatusCode::OK, Json(settings)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "message": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn get_handler(
    Extension(db): Extension<Db>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    match service::get(&db, &key).await {
        Ok(setting) => (StatusCode::OK, Json(setting)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "message": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn update_handler(
    Extension(db): Extension<Db>,
    Path(key): Path<String>,
    Json(dto): Json<UpdateEmailsDto>,
) -> impl IntoResponse {
    match service::update_emails(&db, &key, dto.emails).await {
        Ok(setting) => (StatusCode::OK, Json(setting)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "message": e.to_string() })),
        )
            .into_response(),
    }
}
