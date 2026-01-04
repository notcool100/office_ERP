use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Intern {
    pub id: Uuid,
    pub intern_id: String,
    pub person_id: Uuid,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub supervisor_id: Option<Uuid>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub stipend: Option<sqlx::types::BigDecimal>,
    pub university: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InternWithPerson {
    pub id: Uuid,
    pub intern_id: String,
    pub person_id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub supervisor_id: Option<Uuid>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub stipend: Option<sqlx::types::BigDecimal>,
    pub university: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
