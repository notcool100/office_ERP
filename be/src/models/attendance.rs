use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AttendanceRecord {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub date: NaiveDate,
    pub check_in: Option<NaiveDateTime>,
    pub check_out: Option<NaiveDateTime>,
    pub total_hours: Option<sqlx::types::BigDecimal>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AttendanceWithEmployee {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub employee_name: String,
    pub date: NaiveDate,
    pub check_in: Option<NaiveDateTime>,
    pub check_out: Option<NaiveDateTime>,
    pub total_hours: Option<sqlx::types::BigDecimal>,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
