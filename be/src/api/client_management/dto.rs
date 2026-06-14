use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Clients ──────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ClientDto {
    pub id: Uuid,
    pub name: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub industry: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub status: String,
    pub deliverable_count: i64,
    pub created_by: Option<Uuid>,
    pub created_by_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateClientDto {
    pub name: String,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub industry: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClientDto {
    pub name: Option<String>,
    pub contact_person: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub industry: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListClientsQuery {
    pub status: Option<String>,
    pub search: Option<String>,
}

// ── Deliverables ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DeliverableDto {
    pub id: Uuid,
    pub client_id: Uuid,
    pub client_name: String,
    pub campaign_id: Option<Uuid>,
    pub campaign_name: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub deliverable_type: String,
    pub due_date: Option<NaiveDate>,
    pub status: String,
    pub assigned_to: Option<Uuid>,
    pub assigned_to_name: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_by_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateDeliverableDto {
    pub client_id: Uuid,
    pub campaign_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub deliverable_type: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeliverableDto {
    pub client_id: Option<Uuid>,
    pub campaign_id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub deliverable_type: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ListDeliverablesQuery {
    pub client_id: Option<Uuid>,
    pub campaign_id: Option<Uuid>,
    pub status: Option<String>,
    pub search: Option<String>,
}
