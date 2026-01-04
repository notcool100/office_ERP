use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct LeaveType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub max_days_per_year: Option<i32>,
    pub requires_approval: bool,
    pub carry_forward: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct LeaveRequest {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_days: sqlx::types::BigDecimal,
    pub reason: Option<String>,
    pub status: String,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<NaiveDateTime>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LeaveRequestWithDetails {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub employee_name: String,
    pub leave_type_id: Uuid,
    pub leave_type_name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_days: sqlx::types::BigDecimal,
    pub reason: Option<String>,
    pub status: String,
    pub approved_by: Option<Uuid>,
    pub approver_name: Option<String>,
    pub approved_at: Option<NaiveDateTime>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
