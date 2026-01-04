use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLeaveRequestRequest {
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeaveRequestRequest {
    pub notes: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveRejectLeaveRequest {
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveRequestResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub employee_name: String,
    pub leave_type_id: Uuid,
    pub leave_type_name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_days: f64,
    pub reason: Option<String>,
    pub status: String,
    pub approved_by: Option<Uuid>,
    pub approver_name: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveTypeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub max_days_per_year: Option<i32>,
    pub requires_approval: bool,
    pub carry_forward: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLeaveRequestsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub employee_id: Option<Uuid>,
    pub status: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListLeaveRequestsResponse {
    pub requests: Vec<LeaveRequestResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveBalanceResponse {
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub leave_type_name: String,
    pub total_allowed: i32,
    pub used: f64,
    pub remaining: f64,
}
