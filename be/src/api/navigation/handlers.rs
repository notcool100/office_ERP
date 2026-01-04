use crate::{api::navigation::{dto::*, service}, db::Db};
use crate::models::user::User;
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn create_navigation_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<CreateNavigationItemDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let item = service::create_navigation_item(&db, payload)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let response = NavigationItemResponseDto {
        id: item.id,
        name: item.name,
        path: item.path,
        icon: item.icon,
        parent_id: item.parent_id,
        display_order: item.display_order,
        is_active: item.is_active,
        created_at: item.created_at,
        updated_at: item.updated_at,
    };
    
    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn get_navigation_items_handler(
    Extension(db): Extension<Db>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let is_active = query.get("is_active").and_then(|v| v.parse::<bool>().ok());
    
    let items = service::get_navigation_items(&db, is_active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response: Vec<NavigationItemResponseDto> = items
        .into_iter()
        .map(|item| NavigationItemResponseDto {
            id: item.id,
            name: item.name,
            path: item.path,
            icon: item.icon,
            parent_id: item.parent_id,
            display_order: item.display_order,
            is_active: item.is_active,
            created_at: item.created_at,
            updated_at: item.updated_at,
        })
        .collect();
    
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn get_user_navigation_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let nav_items = service::get_user_navigation(&db, user.id)
        .await
        .map_err(|e| {
            eprintln!("Error getting user navigation: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok((StatusCode::OK, Json(json!(nav_items))))
}

pub async fn get_navigation_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let item = service::get_navigation_item_by_id(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let response = NavigationItemResponseDto {
        id: item.id,
        name: item.name,
        path: item.path,
        icon: item.icon,
        parent_id: item.parent_id,
        display_order: item.display_order,
        is_active: item.is_active,
        created_at: item.created_at,
        updated_at: item.updated_at,
    };
    
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn update_navigation_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateNavigationItemDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let item = service::update_navigation_item(&db, id, payload)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let response = NavigationItemResponseDto {
        id: item.id,
        name: item.name,
        path: item.path,
        icon: item.icon,
        parent_id: item.parent_id,
        display_order: item.display_order,
        is_active: item.is_active,
        created_at: item.created_at,
        updated_at: item.updated_at,
    };
    
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn delete_navigation_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::delete_navigation_item(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Navigation item deleted successfully"})),
    ))
}
