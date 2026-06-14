use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::dto::{
    ClientDto, CreateClientDto, CreateDeliverableDto, DeliverableDto,
    ListClientsQuery, ListDeliverablesQuery, UpdateClientDto, UpdateDeliverableDto,
};
use crate::models::user::User;

const VALID_CLIENT_STATUSES: &[&str] = &["active", "inactive", "prospect"];
const VALID_DELIVERABLE_STATUSES: &[&str] = &["pending", "in_progress", "review", "done", "cancelled"];
const VALID_DELIVERABLE_TYPES: &[&str] = &[
    "social_post", "blog_post", "ad_creative", "report",
    "video", "design", "email_campaign", "other",
];

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("{0}")]
    BadRequest(String),
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}

// ── Clients ──────────────────────────────────────────────────────────────────

const CLIENT_SELECT: &str = r#"
    SELECT
        c.id, c.name, c.contact_person, c.email, c.phone,
        c.industry, c.website, c.address, c.notes, c.status,
        (SELECT COUNT(*) FROM client_deliverables cd WHERE cd.client_id = c.id) AS deliverable_count,
        c.created_by, u.user_name AS created_by_name,
        c.created_at, c.updated_at
    FROM clients c
    LEFT JOIN users u ON u.id = c.created_by
"#;

pub async fn list_clients(
    pool: &PgPool,
    query: ListClientsQuery,
) -> Result<Vec<ClientDto>, ClientError> {
    let rows = sqlx::query_as::<_, ClientDto>(&format!(
        "{} WHERE ($1::text IS NULL OR c.status = $1)
              AND ($2::text IS NULL OR c.name ILIKE $2 OR c.contact_person ILIKE $2
                   OR c.industry ILIKE $2 OR c.email ILIKE $2)
         ORDER BY c.name ASC",
        CLIENT_SELECT
    ))
    .bind(query.status)
    .bind(query.search.map(|s| format!("%{}%", s)))
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_client(pool: &PgPool, id: Uuid) -> Result<ClientDto, ClientError> {
    sqlx::query_as::<_, ClientDto>(&format!("{} WHERE c.id = $1", CLIENT_SELECT))
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(ClientError::NotFound)
}

pub async fn create_client(
    pool: &PgPool,
    user: &User,
    dto: CreateClientDto,
) -> Result<ClientDto, ClientError> {
    if dto.name.trim().is_empty() {
        return Err(ClientError::BadRequest("Client name is required".into()));
    }
    let status = dto.status.as_deref().unwrap_or("active");
    if !VALID_CLIENT_STATUSES.contains(&status) {
        return Err(ClientError::BadRequest(format!("Invalid status: {}", status)));
    }

    let now = Utc::now().naive_utc();
    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO clients
            (name, contact_person, email, phone, industry, website, address, notes, status, created_by, created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$11)
        RETURNING id
        "#,
    )
    .bind(dto.name.trim())
    .bind(dto.contact_person)
    .bind(dto.email)
    .bind(dto.phone)
    .bind(dto.industry)
    .bind(dto.website)
    .bind(dto.address)
    .bind(dto.notes)
    .bind(status)
    .bind(user.id)
    .bind(now)
    .fetch_one(pool)
    .await?;

    get_client(pool, id).await
}

pub async fn update_client(
    pool: &PgPool,
    _user: &User,
    id: Uuid,
    dto: UpdateClientDto,
) -> Result<ClientDto, ClientError> {
    get_client(pool, id).await?; // ensures exists

    if let Some(ref s) = dto.status {
        if !VALID_CLIENT_STATUSES.contains(&s.as_str()) {
            return Err(ClientError::BadRequest(format!("Invalid status: {}", s)));
        }
    }

    sqlx::query(
        r#"
        UPDATE clients SET
            name           = COALESCE($1,  name),
            contact_person = COALESCE($2,  contact_person),
            email          = COALESCE($3,  email),
            phone          = COALESCE($4,  phone),
            industry       = COALESCE($5,  industry),
            website        = COALESCE($6,  website),
            address        = COALESCE($7,  address),
            notes          = COALESCE($8,  notes),
            status         = COALESCE($9,  status),
            updated_at     = $10
        WHERE id = $11
        "#,
    )
    .bind(dto.name.map(|n| n.trim().to_string()))
    .bind(dto.contact_person)
    .bind(dto.email)
    .bind(dto.phone)
    .bind(dto.industry)
    .bind(dto.website)
    .bind(dto.address)
    .bind(dto.notes)
    .bind(dto.status)
    .bind(Utc::now().naive_utc())
    .bind(id)
    .execute(pool)
    .await?;

    get_client(pool, id).await
}

pub async fn delete_client(pool: &PgPool, id: Uuid) -> Result<(), ClientError> {
    let rows = sqlx::query("DELETE FROM clients WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if rows.rows_affected() == 0 {
        return Err(ClientError::NotFound);
    }
    Ok(())
}

// ── Deliverables ─────────────────────────────────────────────────────────────

const DELIVERABLE_SELECT: &str = r#"
    SELECT
        d.id, d.client_id, cl.name AS client_name,
        d.campaign_id, ca.name AS campaign_name,
        d.title, d.description, d.deliverable_type, d.due_date, d.status,
        d.assigned_to, au.user_name AS assigned_to_name,
        d.created_by, cu.user_name AS created_by_name,
        d.created_at, d.updated_at
    FROM client_deliverables d
    JOIN clients cl ON cl.id = d.client_id
    LEFT JOIN campaigns ca ON ca.id = d.campaign_id
    LEFT JOIN users au ON au.id = d.assigned_to
    LEFT JOIN users cu ON cu.id = d.created_by
"#;

pub async fn list_deliverables(
    pool: &PgPool,
    query: ListDeliverablesQuery,
) -> Result<Vec<DeliverableDto>, ClientError> {
    let rows = sqlx::query_as::<_, DeliverableDto>(&format!(
        "{} WHERE ($1::uuid IS NULL OR d.client_id = $1)
              AND ($2::uuid IS NULL OR d.campaign_id = $2)
              AND ($3::text IS NULL OR d.status = $3)
              AND ($4::text IS NULL OR d.title ILIKE $4 OR d.description ILIKE $4)
         ORDER BY d.due_date ASC NULLS LAST, d.created_at DESC",
        DELIVERABLE_SELECT
    ))
    .bind(query.client_id)
    .bind(query.campaign_id)
    .bind(query.status)
    .bind(query.search.map(|s| format!("%{}%", s)))
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_deliverable(pool: &PgPool, id: Uuid) -> Result<DeliverableDto, ClientError> {
    sqlx::query_as::<_, DeliverableDto>(&format!("{} WHERE d.id = $1", DELIVERABLE_SELECT))
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(ClientError::NotFound)
}

pub async fn create_deliverable(
    pool: &PgPool,
    user: &User,
    dto: CreateDeliverableDto,
) -> Result<DeliverableDto, ClientError> {
    if dto.title.trim().is_empty() {
        return Err(ClientError::BadRequest("Title is required".into()));
    }

    let d_type = dto.deliverable_type.as_deref().unwrap_or("other");
    if !VALID_DELIVERABLE_TYPES.contains(&d_type) {
        return Err(ClientError::BadRequest(format!("Invalid type: {}", d_type)));
    }
    let status = dto.status.as_deref().unwrap_or("pending");
    if !VALID_DELIVERABLE_STATUSES.contains(&status) {
        return Err(ClientError::BadRequest(format!("Invalid status: {}", status)));
    }

    let now = Utc::now().naive_utc();
    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO client_deliverables
            (client_id, campaign_id, title, description, deliverable_type, due_date, status, assigned_to, created_by, created_at, updated_at)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$10)
        RETURNING id
        "#,
    )
    .bind(dto.client_id)
    .bind(dto.campaign_id)
    .bind(dto.title.trim())
    .bind(dto.description)
    .bind(d_type)
    .bind(dto.due_date)
    .bind(status)
    .bind(dto.assigned_to)
    .bind(user.id)
    .bind(now)
    .fetch_one(pool)
    .await?;

    get_deliverable(pool, id).await
}

pub async fn update_deliverable(
    pool: &PgPool,
    id: Uuid,
    dto: UpdateDeliverableDto,
) -> Result<DeliverableDto, ClientError> {
    get_deliverable(pool, id).await?;

    if let Some(ref t) = dto.deliverable_type {
        if !VALID_DELIVERABLE_TYPES.contains(&t.as_str()) {
            return Err(ClientError::BadRequest(format!("Invalid type: {}", t)));
        }
    }
    if let Some(ref s) = dto.status {
        if !VALID_DELIVERABLE_STATUSES.contains(&s.as_str()) {
            return Err(ClientError::BadRequest(format!("Invalid status: {}", s)));
        }
    }

    sqlx::query(
        r#"
        UPDATE client_deliverables SET
            client_id        = COALESCE($1,  client_id),
            campaign_id      = COALESCE($2,  campaign_id),
            title            = COALESCE($3,  title),
            description      = COALESCE($4,  description),
            deliverable_type = COALESCE($5,  deliverable_type),
            due_date         = COALESCE($6,  due_date),
            status           = COALESCE($7,  status),
            assigned_to      = COALESCE($8,  assigned_to),
            updated_at       = $9
        WHERE id = $10
        "#,
    )
    .bind(dto.client_id)
    .bind(dto.campaign_id)
    .bind(dto.title.map(|t| t.trim().to_string()))
    .bind(dto.description)
    .bind(dto.deliverable_type)
    .bind(dto.due_date)
    .bind(dto.status)
    .bind(dto.assigned_to)
    .bind(Utc::now().naive_utc())
    .bind(id)
    .execute(pool)
    .await?;

    get_deliverable(pool, id).await
}

pub async fn delete_deliverable(pool: &PgPool, id: Uuid) -> Result<(), ClientError> {
    let rows = sqlx::query("DELETE FROM client_deliverables WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if rows.rows_affected() == 0 {
        return Err(ClientError::NotFound);
    }
    Ok(())
}
