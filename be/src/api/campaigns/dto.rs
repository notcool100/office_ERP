use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CampaignDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub platform: String,
    pub status: String,
    pub budget: BigDecimal,
    pub spent: BigDecimal,
    pub revenue: BigDecimal,
    pub roi: Option<BigDecimal>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub target_audience: Option<String>,
    pub goals: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_by_name: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub assigned_to_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignDto {
    pub name: String,
    pub description: Option<String>,
    pub platform: String,
    pub status: Option<String>,
    pub budget: Option<BigDecimal>,
    pub spent: Option<BigDecimal>,
    pub revenue: Option<BigDecimal>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub target_audience: Option<String>,
    pub goals: Option<String>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCampaignDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub platform: Option<String>,
    pub status: Option<String>,
    pub budget: Option<BigDecimal>,
    pub spent: Option<BigDecimal>,
    pub revenue: Option<BigDecimal>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub target_audience: Option<String>,
    pub goals: Option<String>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ListCampaignsQuery {
    pub platform: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
}
