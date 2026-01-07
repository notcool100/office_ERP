use uuid::Uuid;
use anyhow::{Result, anyhow};
use crate::{
    api::{auth::password::hash_password, user::dto::CreateUserRequest},
    db::Db,
    models::user::User,
};

pub async fn get_by_id(db: &Db, id: Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await
}

pub async fn list_users(db: &Db) -> Result<Vec<User>, axum::http::StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(users)
}

pub async fn create_user(db: &Db, req: CreateUserRequest) -> Result<User> {
    // Check if user already exists for this person
    let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE person_id = $1)")
        .bind(req.person_id)
        .fetch_one(db)
        .await
        .map_err(|e| anyhow!("Database error: {}", e))?;

    if exists {
        return Err(anyhow!("User already exists for this person"));
    }

    // Check if username already exists
    let username_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE user_name = $1)")
        .bind(&req.user_name)
        .fetch_one(db)
        .await
        .map_err(|e| anyhow!("Database error: {}", e))?;

    if username_exists {
        return Err(anyhow!("Username already taken"));
    }

    let hashed = hash_password(&req.password)?;
    let user_id = Uuid::new_v4();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, user_name, email, phone, password_hash, person_id, is_admin, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&req.user_name)
    .bind(&req.email)
    .bind(&req.phone)
    .bind(hashed)
    .bind(req.person_id)
    .bind(req.is_admin.unwrap_or(false))
    .fetch_one(db)
    .await
    .map_err(|e| anyhow!("Failed to create user: {}", e))?;

    Ok(user)
}

pub async fn update_user(
    db: &Db,
    id: Uuid,
    user_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    is_admin: Option<bool>,
) -> Result<User> {
    // Check if username is being changed and if it's already taken
    if let Some(ref username) = user_name {
        let username_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM users WHERE user_name = $1 AND id != $2)"
        )
        .bind(username)
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|e| anyhow!("Database error: {}", e))?;

        if username_exists {
            return Err(anyhow!("Username already taken"));
        }
    }

    // Build dynamic update query
    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET 
            user_name = COALESCE($1, user_name),
            email = COALESCE($2, email),
            phone = COALESCE($3, phone),
            is_admin = COALESCE($4, is_admin)
        WHERE id = $5
        RETURNING *
        "#,
    )
    .bind(user_name)
    .bind(email)
    .bind(phone)
    .bind(is_admin)
    .bind(id)
    .fetch_one(db)
    .await
    .map_err(|e| anyhow!("Failed to update user: {}", e))?;

    Ok(user)
}

pub async fn delete_user(db: &Db, id: Uuid) -> Result<()> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| anyhow!("Failed to delete user: {}", e))?;

    if result.rows_affected() == 0 {
        return Err(anyhow!("User not found"));
    }

    Ok(())
}

pub async fn change_password(db: &Db, id: Uuid, new_password: String) -> Result<()> {
    let hashed = hash_password(&new_password)?;

    let result = sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
        .bind(hashed)
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| anyhow!("Failed to change password: {}", e))?;

    if result.rows_affected() == 0 {
        return Err(anyhow!("User not found"));
    }

    Ok(())
}
