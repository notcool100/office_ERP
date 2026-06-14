use bigdecimal::BigDecimal;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::dto::{CampaignDto, CreateCampaignDto, ListCampaignsQuery, UpdateCampaignDto};
use crate::models::user::User;

const VALID_STATUSES: &[&str] = &["draft", "active", "paused", "completed", "cancelled"];

const VALID_PLATFORMS: &[&str] = &[
    "google_ads", "facebook", "instagram", "linkedin",
    "tiktok", "youtube", "email", "twitter", "other",
];

#[derive(Debug, thiserror::Error)]
pub enum CampaignError {
    #[error("Campaign not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("{0}")]
    BadRequest(String),
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

const SELECT: &str = r#"
    SELECT
        c.id, c.name, c.description, c.platform, c.status,
        c.budget, c.spent, c.revenue,
        CASE WHEN c.spent > 0
             THEN ROUND(((c.revenue - c.spent) / c.spent * 100)::numeric, 2)
             ELSE NULL END AS roi,
        c.start_date, c.end_date, c.target_audience, c.goals,
        c.created_by, cb.user_name AS created_by_name,
        c.assigned_to, ab.user_name AS assigned_to_name,
        c.created_at, c.updated_at
    FROM campaigns c
    LEFT JOIN users cb ON cb.id = c.created_by
    LEFT JOIN users ab ON ab.id = c.assigned_to
"#;

pub async fn list_campaigns(
    pool: &PgPool,
    query: ListCampaignsQuery,
) -> Result<Vec<CampaignDto>, CampaignError> {
    let rows = sqlx::query_as::<_, CampaignDto>(&format!(
        "{} WHERE ($1::text IS NULL OR c.platform = $1)
              AND ($2::text IS NULL OR c.status = $2)
              AND ($3::text IS NULL OR c.name ILIKE $3 OR c.description ILIKE $3)
         ORDER BY c.created_at DESC",
        SELECT
    ))
    .bind(query.platform)
    .bind(query.status)
    .bind(query.search.map(|s| format!("%{}%", s)))
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_campaign(pool: &PgPool, id: Uuid) -> Result<CampaignDto, CampaignError> {
    sqlx::query_as::<_, CampaignDto>(&format!("{} WHERE c.id = $1", SELECT))
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(CampaignError::NotFound)
}

pub async fn create_campaign(
    pool: &PgPool,
    user: &User,
    dto: CreateCampaignDto,
) -> Result<CampaignDto, CampaignError> {
    if dto.name.trim().is_empty() {
        return Err(CampaignError::BadRequest("Name is required".into()));
    }
    if !VALID_PLATFORMS.contains(&dto.platform.as_str()) {
        return Err(CampaignError::BadRequest(format!("Invalid platform: {}", dto.platform)));
    }
    let status = dto.status.as_deref().unwrap_or("draft");
    if !VALID_STATUSES.contains(&status) {
        return Err(CampaignError::BadRequest(format!("Invalid status: {}", status)));
    }

    let zero = BigDecimal::from(0);
    let now = Utc::now().naive_utc();

    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO campaigns
            (name, description, platform, status, budget, spent, revenue,
             start_date, end_date, target_audience, goals, created_by, assigned_to,
             created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$14)
        RETURNING id
        "#,
    )
    .bind(dto.name.trim())
    .bind(dto.description)
    .bind(&dto.platform)
    .bind(status)
    .bind(dto.budget.unwrap_or_else(|| zero.clone()))
    .bind(dto.spent.unwrap_or_else(|| zero.clone()))
    .bind(dto.revenue.unwrap_or(zero))
    .bind(dto.start_date)
    .bind(dto.end_date)
    .bind(dto.target_audience)
    .bind(dto.goals)
    .bind(user.id)
    .bind(dto.assigned_to)
    .bind(now)
    .fetch_one(pool)
    .await?;

    get_campaign(pool, id).await
}

pub async fn update_campaign(
    pool: &PgPool,
    user: &User,
    id: Uuid,
    dto: UpdateCampaignDto,
) -> Result<CampaignDto, CampaignError> {
    let existing = get_campaign(pool, id).await?;

    if !user.is_admin && existing.created_by != Some(user.id) {
        return Err(CampaignError::Forbidden);
    }

    if let Some(ref p) = dto.platform {
        if !VALID_PLATFORMS.contains(&p.as_str()) {
            return Err(CampaignError::BadRequest(format!("Invalid platform: {}", p)));
        }
    }
    if let Some(ref s) = dto.status {
        if !VALID_STATUSES.contains(&s.as_str()) {
            return Err(CampaignError::BadRequest(format!("Invalid status: {}", s)));
        }
    }

    sqlx::query(
        r#"
        UPDATE campaigns SET
            name            = COALESCE($1,  name),
            description     = COALESCE($2,  description),
            platform        = COALESCE($3,  platform),
            status          = COALESCE($4,  status),
            budget          = COALESCE($5,  budget),
            spent           = COALESCE($6,  spent),
            revenue         = COALESCE($7,  revenue),
            start_date      = COALESCE($8,  start_date),
            end_date        = COALESCE($9,  end_date),
            target_audience = COALESCE($10, target_audience),
            goals           = COALESCE($11, goals),
            assigned_to     = COALESCE($12, assigned_to),
            updated_at      = $13
        WHERE id = $14
        "#,
    )
    .bind(dto.name.map(|n| n.trim().to_string()))
    .bind(dto.description)
    .bind(dto.platform)
    .bind(dto.status)
    .bind(dto.budget)
    .bind(dto.spent)
    .bind(dto.revenue)
    .bind(dto.start_date)
    .bind(dto.end_date)
    .bind(dto.target_audience)
    .bind(dto.goals)
    .bind(dto.assigned_to)
    .bind(Utc::now().naive_utc())
    .bind(id)
    .execute(pool)
    .await?;

    get_campaign(pool, id).await
}

pub async fn delete_campaign(
    pool: &PgPool,
    user: &User,
    id: Uuid,
) -> Result<(), CampaignError> {
    let existing = get_campaign(pool, id).await?;

    if !user.is_admin && existing.created_by != Some(user.id) {
        return Err(CampaignError::Forbidden);
    }

    sqlx::query("DELETE FROM campaigns WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
