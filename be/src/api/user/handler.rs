use axum::{Extension, Json, http::StatusCode};
use crate::{
    api::user::{dto::CreateUserRequest, service},
    db::Db,
    models::user::User,
};

pub async fn list_users_handler(
    Extension(db): Extension<Db>,
) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    let users = service::list_users(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(users)))
}

pub async fn create_user_handler(
    Extension(db): Extension<Db>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = service::create_user(&db, payload)
        .await
        .map_err(|e| {
            eprintln!("Error creating user: {}", e);
            StatusCode::BAD_REQUEST
        })?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user_handler(
    Extension(db): Extension<Db>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = service::get_by_id(&db, id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn update_user_handler(
    Extension(db): Extension<Db>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<crate::api::user::dto::UpdateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = service::update_user(
        &db,
        id,
        payload.user_name,
        payload.email,
        payload.phone,
        payload.is_admin,
    )
    .await
    .map_err(|e| {
        eprintln!("Error updating user: {}", e);
        StatusCode::BAD_REQUEST
    })?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete_user_handler(
    Extension(db): Extension<Db>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> Result<StatusCode, StatusCode> {
    service::delete_user(&db, id)
        .await
        .map_err(|e| {
            eprintln!("Error deleting user: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn change_password_handler(
    Extension(db): Extension<Db>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(payload): Json<crate::api::user::dto::ChangePasswordRequest>,
) -> Result<StatusCode, StatusCode> {
    service::change_password(&db, id, payload.new_password)
        .await
        .map_err(|e| {
            eprintln!("Error changing password: {}", e);
            StatusCode::BAD_REQUEST
        })?;
    Ok(StatusCode::OK)
}
