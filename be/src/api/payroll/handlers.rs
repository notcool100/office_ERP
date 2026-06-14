use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;

use crate::{db::Db, models::user::User};

use super::{
    dto::{CreatePayrollRunRequest, SalaryStructureRequest, UpdatePayrollEntryRequest},
    service,
};

fn service_err(e: anyhow::Error) -> (StatusCode, Json<serde_json::Value>) {
    let msg = e.to_string();
    let status = if msg.contains("not found") {
        StatusCode::NOT_FOUND
    } else if msg.contains("already exists") || msg.contains("Cannot") {
        StatusCode::CONFLICT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };
    (status, Json(json!({ "error": msg })))
}

// ── Salary Structures ─────────────────────────────────────────────────────────

pub async fn list_salary_structures_handler(
    Extension(db): Extension<Db>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::list_salary_structures(&db)
        .await
        .map(|rows| (StatusCode::OK, Json(json!(rows))))
        .map_err(service_err)
}

pub async fn create_salary_structure_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<SalaryStructureRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::create_salary_structure(&db, payload, user.id)
        .await
        .map(|row| (StatusCode::CREATED, Json(json!(row))))
        .map_err(service_err)
}

pub async fn delete_salary_structure_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::delete_salary_structure(&db, id)
        .await
        .map(|_| (StatusCode::OK, Json(json!({ "message": "Deleted" }))))
        .map_err(service_err)
}

// ── Payroll Runs ──────────────────────────────────────────────────────────────

pub async fn list_payroll_runs_handler(
    Extension(db): Extension<Db>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::list_payroll_runs(&db)
        .await
        .map(|rows| (StatusCode::OK, Json(json!(rows))))
        .map_err(service_err)
}

pub async fn create_payroll_run_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreatePayrollRunRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::create_payroll_run(&db, payload, user.id)
        .await
        .map(|run| (StatusCode::CREATED, Json(json!(run))))
        .map_err(service_err)
}

pub async fn get_payroll_run_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::get_payroll_run_detail(&db, id)
        .await
        .map(|detail| (StatusCode::OK, Json(json!(detail))))
        .map_err(service_err)
}

pub async fn process_payroll_run_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::process_payroll_run(&db, id)
        .await
        .map(|detail| (StatusCode::OK, Json(json!(detail))))
        .map_err(service_err)
}

pub async fn mark_run_paid_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::mark_run_paid(&db, id)
        .await
        .map(|run| (StatusCode::OK, Json(json!(run))))
        .map_err(service_err)
}

pub async fn delete_payroll_run_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::delete_payroll_run(&db, id)
        .await
        .map(|_| (StatusCode::OK, Json(json!({ "message": "Deleted" }))))
        .map_err(service_err)
}

// ── Entry Adjustments ─────────────────────────────────────────────────────────

pub async fn update_payroll_entry_handler(
    Extension(db): Extension<Db>,
    Path(entry_id): Path<Uuid>,
    Json(payload): Json<UpdatePayrollEntryRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::update_payroll_entry(&db, entry_id, payload)
        .await
        .map(|entry| (StatusCode::OK, Json(json!(entry))))
        .map_err(service_err)
}

// ── My Payslips (no RBAC — any authenticated user) ───────────────────────────

pub async fn my_payslips_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    service::get_my_payslips(&db, user.id)
        .await
        .map(|slips| (StatusCode::OK, Json(json!(slips))))
        .map_err(service_err)
}
