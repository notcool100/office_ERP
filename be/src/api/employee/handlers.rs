use crate::{
    api::employee::{
        dto::{CreateEmployeeRequest, ListEmployeesQuery, UpdateEmployeeRequest, UpdateFaceDescriptorRequest},
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

pub async fn create_employee_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<CreateEmployeeRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    match service::create_employee(&db, payload).await {
        Ok(employee) => Ok((StatusCode::CREATED, Json(json!(employee)))),
        Err(e) => {
            eprintln!("Error creating employee: {}", e);
            // Return the error message to the client
            Ok((StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() }))))
        }
    }
}

pub async fn get_employee_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let employee = service::get_employee(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(json!(employee))))
}

pub async fn list_employees_handler(
    Extension(db): Extension<Db>,
    Query(query): Query<ListEmployeesQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let response = service::list_employees(&db, query)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn update_employee_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateEmployeeRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let employee = service::update_employee(&db, id, payload)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(json!(employee))))
}

pub async fn delete_employee_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::delete_employee(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Employee deleted successfully"})),
    ))
}

pub async fn update_face_descriptor_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateFaceDescriptorRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::update_face_descriptor(&db, id, payload.descriptor)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((
        StatusCode::OK,
        Json(json!({"message": "Face descriptor updated successfully"})),
    ))
}

pub async fn list_face_descriptors_handler(
    Extension(db): Extension<Db>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let descriptors = service::get_all_face_descriptors(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(json!(descriptors))))
}

