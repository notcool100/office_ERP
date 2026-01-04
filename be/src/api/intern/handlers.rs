use crate::{
    api::intern::{
        dto::{CreateInternRequest, ListInternsQuery, UpdateInternRequest},
        service,
    },
    db::Db,
};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

pub async fn create_intern_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<CreateInternRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let intern = service::create_intern(&db, payload)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok((StatusCode::CREATED, Json(json!(intern))))
}

pub async fn get_intern_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let intern = service::get_intern(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(json!(intern))))
}

pub async fn list_interns_handler(
    Extension(db): Extension<Db>,
    Query(query): Query<ListInternsQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let response = service::list_interns(&db, query)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn update_intern_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateInternRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let intern = service::update_intern(&db, id, payload)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(json!(intern))))
}

pub async fn delete_intern_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::delete_intern(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Intern deleted successfully"})),
    ))
}
