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
