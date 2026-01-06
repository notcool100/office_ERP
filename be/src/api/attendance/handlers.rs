use crate::{
    api::attendance::{dto::{CheckInRequest, CheckOutRequest, ListAttendanceQuery}, service},
    db::Db,
};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde_json::json;


pub async fn check_in_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<CheckInRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    match service::check_in(&db, payload).await {
        Ok(attendance) => Ok((StatusCode::CREATED, Json(json!(attendance)))),
        Err(e) => {
            eprintln!("Error checking in: {}", e);
            Ok((StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
        }
    }
}

pub async fn check_out_handler(
    Extension(db): Extension<Db>,
    Path(employee_id): Path<String>,
    Json(payload): Json<CheckOutRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    match service::check_out(&db, employee_id, payload).await {
        Ok(attendance) => Ok((StatusCode::OK, Json(json!(attendance)))),
        Err(e) => {
            eprintln!("Error checking out: {}", e);
            Ok((StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
        }
    }
}

pub async fn list_attendance_handler(
    Extension(db): Extension<Db>,
    Query(query): Query<ListAttendanceQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let response = service::get_attendance_records(&db, query)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn get_attendance_summary_handler(
    Extension(db): Extension<Db>,
    Path((employee_id, start_date, end_date)): Path<(String, NaiveDate, NaiveDate)>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    match service::get_attendance_summary(&db, employee_id, start_date, end_date).await {
        Ok(summary) => Ok((StatusCode::OK, Json(json!(summary)))),
        Err(e) => {
            eprintln!("Error getting summary: {}", e);
            Ok((StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
        }
    }
}
