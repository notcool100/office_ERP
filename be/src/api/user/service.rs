use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{db::Db, models::user::User};

pub fn get_by_email(id: Uuid) -> Option<User> {
    Some(User {
        email: "test@gmail.com".to_string(),
        password_hash: "$2b$12$Gwf0uvxH3L7JLfo0CC/NCOoijK2vQ/wbgP.LeNup8vj6gg31IiFkm".to_string(),
        id,
        user_name: "test_user".to_string(),
        phone: "9876543210".to_string(),
        person_id: Uuid::new_v4(),
        created_at: NaiveDateTime::default(),
    })
}

pub async fn list_users(db: &Db) -> Result<Vec<User>, axum::http::StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT id, user_name FROM users")
        .fetch_all(db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(users)
}
