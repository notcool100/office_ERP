use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use super::{
    dto::{CreateCampaignDto, ListCampaignsQuery, UpdateCampaignDto},
    service,
};
use crate::{db::Db, models::user::User};

fn service_err(e: service::CampaignError) -> Response {
    match e {
        service::CampaignError::NotFound => {
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
        service::CampaignError::Forbidden => {
            (StatusCode::FORBIDDEN, e.to_string()).into_response()
        }
        service::CampaignError::BadRequest(msg) => {
            (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
        }
        service::CampaignError::Database(err) => {
            tracing::error!("DB error in campaigns: {err}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

pub async fn list_campaigns_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Query(query): Query<ListCampaignsQuery>,
) -> Response {
    match service::list_campaigns(&db, query).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn get_campaign_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response {
    match service::get_campaign(&db, id).await {
        Ok(item) => Json(item).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn create_campaign_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(dto): Json<CreateCampaignDto>,
) -> Response {
    match service::create_campaign(&db, &user, dto).await {
        Ok(item) => (StatusCode::CREATED, Json(item)).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn update_campaign_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateCampaignDto>,
) -> Response {
    match service::update_campaign(&db, &user, id, dto).await {
        Ok(item) => Json(item).into_response(),
        Err(e) => service_err(e),
    }
}

pub async fn delete_campaign_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Response {
    match service::delete_campaign(&db, &user, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => service_err(e),
    }
}
