use crate::{api::permissions::{dto::*, service}, db::Db};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn assign_permission_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<AssignPermissionDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::assign_permission(&db, payload)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Permission assigned successfully"})),
    ))
}

pub async fn get_permissions_handler(
    Extension(db): Extension<Db>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let department_id = query.get("department_id").and_then(|v| Uuid::parse_str(v).ok());
    let position_id = query.get("position_id").and_then(|v| Uuid::parse_str(v).ok());
    let navigation_item_id = query.get("navigation_item_id").and_then(|v| Uuid::parse_str(v).ok());
    
    let permissions = service::get_permissions(&db, department_id, position_id, navigation_item_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::OK, Json(json!(permissions))))
}

pub async fn delete_permission_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::delete_permission(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Permission deleted successfully"})),
    ))
}
