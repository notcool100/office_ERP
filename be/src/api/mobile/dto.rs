use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::{
    attendance::dto::AttendanceResponse, leave::dto::LeaveRequestResponse,
    meetings::dto::MeetingResponse,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileUser {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
    pub phone: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobilePerson {
    pub id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub full_name: String,
    pub initials: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileEmployee {
    pub id: Uuid,
    /// Human-readable code (ADY-0142), which is what the attendance
    /// service keys check-in/out on.
    pub employee_code: String,
    pub department: Option<String>,
    pub position: Option<String>,
    pub hire_date: NaiveDate,
    pub employment_type: Option<String>,
    pub status: String,
    pub manager_id: Option<Uuid>,
    pub manager_name: Option<String>,
}

/// What the app is allowed to show. Sent once at startup so the UI can hide
/// tabs the caller would only get a 403 from.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileCapabilities {
    pub company_documents: bool,
    pub client_documents: bool,
    /// True when at least one employee reports to the caller, or they're an
    /// admin — drives the "Awaiting your approval" section.
    pub leave_approvals: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileOffice {
    pub name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub location_name: Option<String>,
    /// Mirrors the server-side geofence so the app can show "within
    /// geofence" before it even tries to check in. The server still
    /// enforces it — this is a hint, not a gate.
    pub geofence_meters: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapResponse {
    pub user: MobileUser,
    pub person: MobilePerson,
    pub employee: Option<MobileEmployee>,
    pub capabilities: MobileCapabilities,
    pub office: MobileOffice,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthSummary {
    pub present_days: i64,
    pub late_days: i64,
    pub absent_days: i64,
    pub leave_days: f64,
    pub total_hours: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardEvent {
    pub id: Uuid,
    pub title: String,
    pub location: Option<String>,
    pub start_at: chrono::NaiveDateTime,
    pub end_at: chrono::NaiveDateTime,
    pub all_day: bool,
    pub scope: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardResponse {
    /// Today in Nepal time, which is the day the attendance module works in.
    pub date: NaiveDate,
    pub greeting_name: String,
    pub attendance: Option<AttendanceResponse>,
    pub month_summary: MonthSummary,
    pub events: Vec<DashboardEvent>,
    pub meetings: Vec<MeetingResponse>,
    pub pending_approvals: Vec<LeaveRequestResponse>,
    pub unread_notifications: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileCheckInRequest {
    /// Base64 JPEG of the check-in selfie, without a data: prefix.
    pub image: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileCheckOutRequest {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateRangeQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileCreateLeaveRequest {
    pub leave_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveDecisionRequest {
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleDay {
    /// ISO-8601 weekday, 1 = Monday .. 7 = Sunday.
    pub day_of_week: i16,
    pub day_name: String,
    pub is_working: bool,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// One colleague, for starting a direct message. Deliberately thin — the
/// app only needs a name to show and a user id to open a DM with.
#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryEntry {
    pub user_id: Uuid,
    pub display_name: String,
    pub email: String,
    pub department: Option<String>,
    pub position: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentBrowseQuery {
    pub category: Option<String>,
    pub owner_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
}

/// One client, for the "pick a client" step before browsing their
/// documents — deliberately thinner than the client-management module's
/// own DTO, since the app only needs enough to populate a list.
#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ClientOption {
    pub id: Uuid,
    pub name: String,
}
