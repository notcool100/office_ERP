use crate::{api::department::{dto::*, service}, db::Db};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn create_department_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<CreateDepartmentDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let department = service::create_department(&db, payload)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let response = DepartmentResponseDto {
        id: department.id,
        name: department.name,
        description: department.description,
        is_active: department.is_active,
        created_at: department.created_at,
        updated_at: department.updated_at,
    };
    
    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn get_departments_handler(
    Extension(db): Extension<Db>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let is_active = query.get("is_active").and_then(|v| v.parse::<bool>().ok());
    
    let departments = service::get_departments(&db, is_active)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response: Vec<DepartmentResponseDto> = departments
        .into_iter()
        .map(|dept| DepartmentResponseDto {
            id: dept.id,
            name: dept.name,
            description: dept.description,
            is_active: dept.is_active,
            created_at: dept.created_at,
            updated_at: dept.updated_at,
        })
        .collect();
    
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn get_department_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let department = service::get_department_by_id(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let response = DepartmentResponseDto {
        id: department.id,
        name: department.name,
        description: department.description,
        is_active: department.is_active,
        created_at: department.created_at,
        updated_at: department.updated_at,
    };
    
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn update_department_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateDepartmentDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let department = service::update_department(&db, id, payload)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    let response = DepartmentResponseDto {
        id: department.id,
        name: department.name,
        description: department.description,
        is_active: department.is_active,
        created_at: department.created_at,
        updated_at: department.updated_at,
    };
    
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn delete_department_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::delete_department(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Department deleted successfully"})),
    ))
}
