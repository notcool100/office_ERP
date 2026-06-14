use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ReportQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub platform: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ReportSummary {
    pub total_campaigns: i64,
    pub active_campaigns: i64,
    pub total_budget: BigDecimal,
    pub total_spent: BigDecimal,
    pub total_revenue: BigDecimal,
    pub overall_roi: Option<BigDecimal>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PlatformStat {
    pub platform: String,
    pub campaign_count: i64,
    pub total_budget: BigDecimal,
    pub total_spent: BigDecimal,
    pub total_revenue: BigDecimal,
    pub roi: Option<BigDecimal>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct StatusStat {
    pub status: String,
    pub count: i64,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CampaignPerformance {
    pub id: Uuid,
    pub name: String,
    pub platform: String,
    pub status: String,
    pub budget: BigDecimal,
    pub spent: BigDecimal,
    pub revenue: BigDecimal,
    pub roi: Option<BigDecimal>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct MarketingReport {
    pub summary: ReportSummary,
    pub by_platform: Vec<PlatformStat>,
    pub by_status: Vec<StatusStat>,
    pub campaigns: Vec<CampaignPerformance>,
}
