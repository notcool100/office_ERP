use axum::{
    extract::{Path, Query, Extension},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;
use crate::db::Db;
use crate::models::user::User; // Auth user

use super::dto::{CreatePersonDto, ListPersonsQuery, UpdatePersonDto};
use super::service;

pub async fn create_person_handler(
    Extension(db): Extension<Db>,
    // Extension(_user): Extension<User>, // Require auth
    Json(payload): Json<CreatePersonDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let person = service::create_person(&db, payload)
        .await
        .map_err(|e| {
            eprintln!("Error creating person: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let response = service::map_person_to_response(person);
    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn list_persons_handler(
    Extension(db): Extension<Db>,
    Query(query): Query<ListPersonsQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let response = service::list_persons(&db, query)
        .await
        .map_err(|e| {
            eprintln!("Error listing persons: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn get_person_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let person = service::get_person_by_id(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let response = service::map_person_to_response(person);
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn update_person_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePersonDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let person = service::update_person(&db, id, payload)
        .await
        .map_err(|e| {
            eprintln!("Error updating person: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let response = service::map_person_to_response(person);
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn delete_person_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::delete_person(&db, id)
        .await
        .map_err(|e| {
            eprintln!("Error deleting person: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Person deleted successfully"})),
    ))
}
