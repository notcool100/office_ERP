use crate::api::daily_log::dto::{CreateDailyLogDto, ListDailyLogQuery, UpdateDailyLogDto};
use crate::api::daily_log::service;
use crate::db::Db;
use crate::models::service_response::ServiceResponse;
use crate::models::user::User;
use axum::{
    Json,
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn create_daily_log(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(dto): Json<CreateDailyLogDto>,
) -> impl IntoResponse {
    match service::create_daily_log(&db, user.id, dto).await {
        Ok(log) => (
            StatusCode::CREATED,
            Json(ServiceResponse::success("Log created", log)),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServiceResponse::error(
                "INTERNAL_ERROR",
                "Failed to create log",
            )),
        ),
    }
}

pub async fn list_daily_logs(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Query(query): Query<ListDailyLogQuery>,
) -> impl IntoResponse {
    // Note: In a real system, we'd check if the user has permission to see others' logs here
    // But for this demo, we'll let the frontend filter by user_id if they aren't Admin/HR
    match service::list_daily_logs(&db, query).await {
        Ok(logs) => (
            StatusCode::OK,
            Json(ServiceResponse::success("Logs fetched", logs)),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServiceResponse::error(
                "INTERNAL_ERROR",
                "Failed to list logs",
            )),
        ),
    }
}

pub async fn update_daily_log(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<uuid::Uuid>,
    Json(dto): Json<UpdateDailyLogDto>,
) -> impl IntoResponse {
    match service::update_daily_log(&db, user.id, id, dto).await {
        Ok(log) => (
            StatusCode::OK,
            Json(ServiceResponse::success("Log updated", log)),
        ),
        Err(e) if e.to_string() == "Unauthorized" => (
            StatusCode::FORBIDDEN,
            Json(ServiceResponse::error(
                "UNAUTHORIZED",
                "Not allowed to edit this log",
            )),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServiceResponse::error(
                "INTERNAL_ERROR",
                "Failed to update log",
            )),
        ),
    }
}

pub async fn delete_daily_log(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    match service::delete_daily_log(&db, user.id, id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ServiceResponse::success("Log deleted", ())),
        ),
        Err(e) if e.to_string() == "Unauthorized" => (
            StatusCode::FORBIDDEN,
            Json(ServiceResponse::error(
                "UNAUTHORIZED",
                "Not allowed to delete this log",
            )),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServiceResponse::error(
                "INTERNAL_ERROR",
                "Failed to delete log",
            )),
        ),
    }
}
