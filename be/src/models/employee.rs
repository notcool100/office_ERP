use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Employee {
    pub id: Uuid,
    pub employee_id: String,
    pub person_id: Uuid,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub hire_date: NaiveDate,
    pub employment_type: Option<String>,
    pub salary: Option<sqlx::types::BigDecimal>,
    pub manager_id: Option<Uuid>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub face_descriptor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmployeeWithPerson {
    pub id: Uuid,
    pub employee_id: String,
    pub person_id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub hire_date: NaiveDate,
    pub employment_type: Option<String>,
    pub salary: Option<sqlx::types::BigDecimal>,
    pub manager_id: Option<Uuid>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
