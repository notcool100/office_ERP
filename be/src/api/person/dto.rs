use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Deserialize)]
pub struct CreatePersonDto {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePersonDto {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PersonResponseDto {
    pub id: Uuid,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ListPersonsQuery {
    pub search: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListPersonsResponse {
    pub persons: Vec<PersonResponseDto>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}
