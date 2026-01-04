use crate::{api::position::{dto::*, service}, db::Db};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn create_position_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<CreatePositionDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let position = service::create_position(&db, payload)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let response = PositionResponseDto {
        id: position.id,
        name: position.name,
        description: position.description,
        is_active: position.is_active,
        created_at: position.created_at,
        updated_at: position.updated_at,
    };
    
    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn get_positions_handler(
    Extension(db): Extension<Db>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let is_active = query.get("is_active").and_then(|v| v.parse::<bool>().ok());
    
    let positions = service::get_positions(&db, is_active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response: Vec<PositionResponseDto> = positions
        .into_iter()
        .map(|pos| PositionResponseDto {
            id: pos.id,
            name: pos.name,
            description: pos.description,
            is_active: pos.is_active,
            created_at: pos.created_at,
            updated_at: pos.updated_at,
        })
        .collect();
    
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn get_position_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let position = service::get_position_by_id(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let response = PositionResponseDto {
        id: position.id,
        name: position.name,
        description: position.description,
        is_active: position.is_active,
        created_at: position.created_at,
        updated_at: position.updated_at,
    };
    
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn update_position_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePositionDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let position = service::update_position(&db, id, payload)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let response = PositionResponseDto {
        id: position.id,
        name: position.name,
        description: position.description,
        is_active: position.is_active,
        created_at: position.created_at,
        updated_at: position.updated_at,
    };
    
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn delete_position_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::delete_position(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Position deleted successfully"})),
    ))
}
