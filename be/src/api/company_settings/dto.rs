use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CompanySettingsResponseDto {
    pub name: String,
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub location_name: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCompanyDetailsDto {
    pub name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub location_name: Option<String>,
}
