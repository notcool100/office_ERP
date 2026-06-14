use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use super::{
    dto::{
        CreateClientDto, CreateDeliverableDto, ListClientsQuery,
        ListDeliverablesQuery, UpdateClientDto, UpdateDeliverableDto,
    },
    service,
};
use crate::{db::Db, models::user::User};

fn err(e: service::ClientError) -> Response {
    match e {
        service::ClientError::NotFound => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
        service::ClientError::Forbidden => (StatusCode::FORBIDDEN, e.to_string()).into_response(),
        service::ClientError::BadRequest(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response(),
        service::ClientError::Database(e) => {
            tracing::error!("DB error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

// ── Client handlers ───────────────────────────────────────────────────────────

pub async fn list_clients_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Query(q): Query<ListClientsQuery>,
) -> Response {
    match service::list_clients(&db, q).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => err(e),
    }
}

pub async fn get_client_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response {
    match service::get_client(&db, id).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => err(e),
    }
}

pub async fn create_client_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(dto): Json<CreateClientDto>,
) -> Response {
    match service::create_client(&db, &user, dto).await {
        Ok(v) => (StatusCode::CREATED, Json(v)).into_response(),
        Err(e) => err(e),
    }
}

pub async fn update_client_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateClientDto>,
) -> Response {
    match service::update_client(&db, &user, id, dto).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => err(e),
    }
}

pub async fn delete_client_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response {
    match service::delete_client(&db, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => err(e),
    }
}

// ── Deliverable handlers ──────────────────────────────────────────────────────

pub async fn list_deliverables_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Query(q): Query<ListDeliverablesQuery>,
) -> Response {
    match service::list_deliverables(&db, q).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => err(e),
    }
}

pub async fn get_deliverable_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response {
    match service::get_deliverable(&db, id).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => err(e),
    }
}

pub async fn create_deliverable_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(dto): Json<CreateDeliverableDto>,
) -> Response {
    match service::create_deliverable(&db, &user, dto).await {
        Ok(v) => (StatusCode::CREATED, Json(v)).into_response(),
        Err(e) => err(e),
    }
}

pub async fn update_deliverable_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateDeliverableDto>,
) -> Response {
    match service::update_deliverable(&db, id, dto).await {
        Ok(v) => Json(v).into_response(),
        Err(e) => err(e),
    }
}

pub async fn delete_deliverable_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response {
    match service::delete_deliverable(&db, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => err(e),
    }
}
