use sqlx::PgPool;

use super::dto::{
    CampaignPerformance, MarketingReport, PlatformStat, ReportQuery, ReportSummary, StatusStat,
};

#[derive(Debug, thiserror::Error)]
pub enum ReportError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

// Shared WHERE clause fragment — filters applied via bound params in every query.
// $1=start_date $2=end_date $3=platform $4=status
const FILTERS: &str = r#"
    WHERE ($1::date IS NULL OR COALESCE(start_date, created_at::date) >= $1)
      AND ($2::date IS NULL OR COALESCE(end_date,   created_at::date) <= $2)
      AND ($3::text IS NULL OR platform = $3)
      AND ($4::text IS NULL OR status   = $4)
"#;

pub async fn generate_report(
    pool: &PgPool,
    q: ReportQuery,
) -> Result<MarketingReport, ReportError> {
    // Run all four queries in parallel
    let (summary, by_platform, by_status, campaigns) = tokio::try_join!(
        fetch_summary(pool, &q),
        fetch_by_platform(pool, &q),
        fetch_by_status(pool, &q),
        fetch_campaigns(pool, &q),
    )?;

    Ok(MarketingReport { summary, by_platform, by_status, campaigns })
}

async fn fetch_summary(pool: &PgPool, q: &ReportQuery) -> Result<ReportSummary, ReportError> {
    let row = sqlx::query_as::<_, ReportSummary>(&format!(
        r#"
        SELECT
            COUNT(*)::bigint AS total_campaigns,
            COUNT(*) FILTER (WHERE status = 'active')::bigint AS active_campaigns,
            COALESCE(SUM(budget),  0) AS total_budget,
            COALESCE(SUM(spent),   0) AS total_spent,
            COALESCE(SUM(revenue), 0) AS total_revenue,
            CASE WHEN SUM(spent) > 0
                 THEN ROUND((SUM(revenue) - SUM(spent)) / SUM(spent) * 100, 2)
                 ELSE NULL END AS overall_roi
        FROM campaigns
        {}
        "#,
        FILTERS
    ))
    .bind(q.start_date)
    .bind(q.end_date)
    .bind(&q.platform)
    .bind(&q.status)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

async fn fetch_by_platform(
    pool: &PgPool,
    q: &ReportQuery,
) -> Result<Vec<PlatformStat>, ReportError> {
    let rows = sqlx::query_as::<_, PlatformStat>(&format!(
        r#"
        SELECT
            platform,
            COUNT(*)::bigint AS campaign_count,
            COALESCE(SUM(budget),  0) AS total_budget,
            COALESCE(SUM(spent),   0) AS total_spent,
            COALESCE(SUM(revenue), 0) AS total_revenue,
            CASE WHEN SUM(spent) > 0
                 THEN ROUND((SUM(revenue) - SUM(spent)) / SUM(spent) * 100, 2)
                 ELSE NULL END AS roi
        FROM campaigns
        {}
        GROUP BY platform
        ORDER BY total_revenue DESC
        "#,
        FILTERS
    ))
    .bind(q.start_date)
    .bind(q.end_date)
    .bind(&q.platform)
    .bind(&q.status)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

async fn fetch_by_status(pool: &PgPool, q: &ReportQuery) -> Result<Vec<StatusStat>, ReportError> {
    let rows = sqlx::query_as::<_, StatusStat>(&format!(
        r#"
        SELECT status, COUNT(*)::bigint AS count
        FROM campaigns
        {}
        GROUP BY status
        ORDER BY count DESC
        "#,
        FILTERS
    ))
    .bind(q.start_date)
    .bind(q.end_date)
    .bind(&q.platform)
    .bind(&q.status)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

async fn fetch_campaigns(
    pool: &PgPool,
    q: &ReportQuery,
) -> Result<Vec<CampaignPerformance>, ReportError> {
    let rows = sqlx::query_as::<_, CampaignPerformance>(&format!(
        r#"
        SELECT
            id, name, platform, status,
            budget, spent, revenue,
            CASE WHEN spent > 0
                 THEN ROUND((revenue - spent) / spent * 100, 2)
                 ELSE NULL END AS roi,
            start_date, end_date
        FROM campaigns
        {}
        ORDER BY roi DESC NULLS LAST, revenue DESC
        "#,
        FILTERS
    ))
    .bind(q.start_date)
    .bind(q.end_date)
    .bind(&q.platform)
    .bind(&q.status)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
