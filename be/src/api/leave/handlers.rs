use crate::{
    api::leave::{
        dto::{ApproveRejectLeaveRequest, CreateLeaveRequestRequest, ListLeaveRequestsQuery},
        service,
    },
    db::Db,
    models::user::User,
};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

pub async fn create_leave_request_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<CreateLeaveRequestRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let leave_request = service::create_leave_request(&db, payload)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok((StatusCode::CREATED, Json(json!(leave_request))))
}

pub async fn get_leave_request_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let leave_request = service::get_leave_request(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(json!(leave_request))))
}

pub async fn list_leave_requests_handler(
    Extension(db): Extension<Db>,
    Query(query): Query<ListLeaveRequestsQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let response = service::list_leave_requests(&db, query)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn approve_leave_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ApproveRejectLeaveRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let leave_request = service::approve_leave(&db, id, user.id, payload)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(json!(leave_request))))
}

pub async fn reject_leave_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ApproveRejectLeaveRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let leave_request = service::reject_leave(&db, id, user.id, payload)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(json!(leave_request))))
}

pub async fn get_leave_types_handler(
    Extension(db): Extension<Db>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let leave_types = service::get_leave_types(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(json!(leave_types))))
}

pub async fn get_leave_balance_handler(
    Extension(db): Extension<Db>,
    Path(employee_id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let balances = service::get_leave_balance(&db, employee_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(json!(balances))))
}
