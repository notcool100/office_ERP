use axum::{Extension, Json, http::StatusCode};

use crate::{api::user::service, db::Db, models::user::User};

pub async fn list_users_handler(
    Extension(db): Extension<Db>,
) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    let users = service::list_users(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(users)))
}
