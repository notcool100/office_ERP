use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEmployeeRequest {
    pub employee_id: String,
    pub person_id: Uuid,
    pub department: Option<Uuid>,
    pub position: Option<Uuid>,
    pub hire_date: NaiveDate,
    pub employment_type: Option<String>,
    pub salary: Option<f64>,
    pub manager_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEmployeeRequest {
    pub department: Option<Uuid>,
    pub position: Option<Uuid>,
    pub employment_type: Option<String>,
    pub salary: Option<f64>,
    pub manager_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeResponse {
    pub id: Uuid,
    pub employee_id: String,
    pub person_id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department: Option<String>,
    pub position: Option<String>,
    pub hire_date: NaiveDate,
    pub employment_type: Option<String>,
    pub salary: Option<f64>,
    pub manager_id: Option<Uuid>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEmployeesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,
    pub department: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEmployeesResponse {
    pub employees: Vec<EmployeeResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFaceDescriptorRequest {
    pub descriptor: String,
}
