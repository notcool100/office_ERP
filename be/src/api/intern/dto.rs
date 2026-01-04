use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInternRequest {
    pub intern_id: String,
    pub person_id: Uuid,
    pub department: Option<String>,
    pub supervisor_id: Option<Uuid>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub stipend: Option<f64>,
    pub university: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInternRequest {
    pub department: Option<String>,
    pub supervisor_id: Option<Uuid>,
    pub end_date: Option<NaiveDate>,
    pub stipend: Option<f64>,
    pub university: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternResponse {
    pub id: Uuid,
    pub intern_id: String,
    pub person_id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub department: Option<String>,
    pub supervisor_id: Option<Uuid>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub stipend: Option<f64>,
    pub university: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInternsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,
    pub department: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListInternsResponse {
    pub interns: Vec<InternResponse>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}
