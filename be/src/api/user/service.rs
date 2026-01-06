use uuid::Uuid;

use crate::{db::Db, models::user::User};

pub async fn get_by_id(db: &Db, id: Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await
}

pub async fn list_users(db: &Db) -> Result<Vec<User>, axum::http::StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT id, user_name FROM users")
        .fetch_all(db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(users)
}
