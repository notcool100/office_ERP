use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckInRequest {
    pub employee_id: String,
    pub notes: Option<String>,
    pub image: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub method: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckOutRequest {
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttendanceResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub employee_name: String,
    pub date: NaiveDate,
    pub check_in: Option<chrono::DateTime<chrono::Utc>>,
    pub check_out: Option<chrono::DateTime<chrono::Utc>>,
    pub total_hours: Option<f64>,
    pub status: String,
    pub notes: Option<String>,
    pub check_in_image: Option<String>,
    pub check_in_lat: Option<f64>,
    pub check_in_long: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAttendanceQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub employee_id: Option<Uuid>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAttendanceResponse {
    pub records: Vec<AttendanceResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttendanceSummary {
    pub employee_id: Uuid,
    pub employee_name: String,
    pub total_days: i64,
    pub present_days: i64,
    pub late_days: i64,
    pub absent_days: i64,
    pub total_hours: f64,
}
