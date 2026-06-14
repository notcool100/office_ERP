use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::dto::{ContentItemResponseDto, CreateContentItemDto, ListContentItemsQuery, UpdateContentItemDto};
use crate::models::user::User;

const VALID_TYPES: &[&str] = &["post", "story", "reel", "blog", "email", "ad", "video", "campaign"];
const VALID_CHANNELS: &[&str] = &["instagram", "facebook", "linkedin", "twitter", "youtube", "tiktok", "email", "blog", "google_ads"];
const VALID_STATUSES: &[&str] = &["draft", "in_review", "scheduled", "published", "cancelled"];

#[derive(Debug, thiserror::Error)]
pub enum ContentCalendarError {
    #[error("Item not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("{0}")]
    BadRequest(String),
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

fn validate_type(v: &str) -> Result<(), ContentCalendarError> {
    if VALID_TYPES.contains(&v) { Ok(()) } else {
        Err(ContentCalendarError::BadRequest(format!("Invalid content_type: {}", v)))
    }
}

fn validate_channel(v: &str) -> Result<(), ContentCalendarError> {
    if VALID_CHANNELS.contains(&v) { Ok(()) } else {
        Err(ContentCalendarError::BadRequest(format!("Invalid channel: {}", v)))
    }
}

fn validate_status(v: &str) -> Result<(), ContentCalendarError> {
    if VALID_STATUSES.contains(&v) { Ok(()) } else {
        Err(ContentCalendarError::BadRequest(format!("Invalid status: {}", v)))
    }
}

pub async fn list_items(
    pool: &PgPool,
    query: ListContentItemsQuery,
) -> Result<Vec<ContentItemResponseDto>, ContentCalendarError> {
    let items = sqlx::query_as::<_, ContentItemResponseDto>(
        r#"
        SELECT
            ci.id, ci.title, ci.description, ci.content_type, ci.channel,
            ci.scheduled_date, ci.scheduled_time, ci.status, ci.campaign_name,
            ci.assigned_to, ci.notes, ci.created_by, ci.created_at, ci.updated_at,
            u1.user_name AS created_by_name,
            u2.user_name AS assigned_to_name
        FROM content_calendar_items ci
        JOIN users u1 ON u1.id = ci.created_by
        LEFT JOIN users u2 ON u2.id = ci.assigned_to
        WHERE ($1::date IS NULL OR ci.scheduled_date >= $1)
          AND ($2::date IS NULL OR ci.scheduled_date <= $2)
          AND ($3::text IS NULL OR ci.status = $3)
          AND ($4::text IS NULL OR ci.channel = $4)
        ORDER BY ci.scheduled_date ASC, ci.scheduled_time ASC NULLS LAST
        "#,
    )
    .bind(query.start_date)
    .bind(query.end_date)
    .bind(query.status)
    .bind(query.channel)
    .fetch_all(pool)
    .await?;

    Ok(items)
}

pub async fn get_item(pool: &PgPool, id: Uuid) -> Result<ContentItemResponseDto, ContentCalendarError> {
    sqlx::query_as::<_, ContentItemResponseDto>(
        r#"
        SELECT
            ci.id, ci.title, ci.description, ci.content_type, ci.channel,
            ci.scheduled_date, ci.scheduled_time, ci.status, ci.campaign_name,
            ci.assigned_to, ci.notes, ci.created_by, ci.created_at, ci.updated_at,
            u1.user_name AS created_by_name,
            u2.user_name AS assigned_to_name
        FROM content_calendar_items ci
        JOIN users u1 ON u1.id = ci.created_by
        LEFT JOIN users u2 ON u2.id = ci.assigned_to
        WHERE ci.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ContentCalendarError::NotFound)
}

pub async fn create_item(
    pool: &PgPool,
    user: &User,
    dto: CreateContentItemDto,
) -> Result<ContentItemResponseDto, ContentCalendarError> {
    if dto.title.trim().is_empty() {
        return Err(ContentCalendarError::BadRequest("Title is required".into()));
    }
    validate_type(&dto.content_type)?;
    validate_channel(&dto.channel)?;

    let status = dto.status.unwrap_or_else(|| "draft".to_string());
    validate_status(&status)?;

    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO content_calendar_items
            (title, description, content_type, channel, scheduled_date, scheduled_time,
             status, campaign_name, assigned_to, created_by, notes, created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$12)
        RETURNING id
        "#,
    )
    .bind(dto.title.trim())
    .bind(dto.description)
    .bind(dto.content_type)
    .bind(dto.channel)
    .bind(dto.scheduled_date)
    .bind(dto.scheduled_time)
    .bind(status)
    .bind(dto.campaign_name)
    .bind(dto.assigned_to)
    .bind(user.id)
    .bind(dto.notes)
    .bind(Utc::now().naive_utc())
    .fetch_one(pool)
    .await?;

    get_item(pool, id).await
}

pub async fn update_item(
    pool: &PgPool,
    user: &User,
    id: Uuid,
    dto: UpdateContentItemDto,
) -> Result<ContentItemResponseDto, ContentCalendarError> {
    let existing = get_item(pool, id).await?;

    if !user.is_admin && existing.created_by != user.id {
        // Allow update if the user is the creator or admin — others can still update
        // (RBAC at route level already checked update permission)
    }

    if let Some(ref t) = dto.content_type { validate_type(t)?; }
    if let Some(ref c) = dto.channel      { validate_channel(c)?; }
    if let Some(ref s) = dto.status       { validate_status(s)?; }

    sqlx::query(
        r#"
        UPDATE content_calendar_items SET
            title          = COALESCE($1, title),
            description    = COALESCE($2, description),
            content_type   = COALESCE($3, content_type),
            channel        = COALESCE($4, channel),
            scheduled_date = COALESCE($5, scheduled_date),
            scheduled_time = COALESCE($6, scheduled_time),
            status         = COALESCE($7, status),
            campaign_name  = COALESCE($8, campaign_name),
            assigned_to    = COALESCE($9, assigned_to),
            notes          = COALESCE($10, notes),
            updated_at     = $11
        WHERE id = $12
        "#,
    )
    .bind(dto.title.map(|t| t.trim().to_string()))
    .bind(dto.description)
    .bind(dto.content_type)
    .bind(dto.channel)
    .bind(dto.scheduled_date)
    .bind(dto.scheduled_time)
    .bind(dto.status)
    .bind(dto.campaign_name)
    .bind(dto.assigned_to)
    .bind(dto.notes)
    .bind(Utc::now().naive_utc())
    .bind(id)
    .execute(pool)
    .await?;

    get_item(pool, id).await
}

pub async fn delete_item(
    pool: &PgPool,
    user: &User,
    id: Uuid,
) -> Result<(), ContentCalendarError> {
    let existing = get_item(pool, id).await?;

    if !user.is_admin && existing.created_by != user.id {
        return Err(ContentCalendarError::Forbidden);
    }

    let result = sqlx::query("DELETE FROM content_calendar_items WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ContentCalendarError::NotFound);
    }

    Ok(())
}
