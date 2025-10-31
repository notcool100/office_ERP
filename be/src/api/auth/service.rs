use anyhow::{Result, anyhow};
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::{
    REFRESH_TOKEN_TTL_DAYS,
    api::auth::{
        dto::{
            AuthResponse, ChangePasswordRequest, ForgotPasswordRequest, LoginRequest,
            RefreshRequest, RegisterRequest,
        },
        email::{generate_verification_token, send_email_stub, token_expiry_time},
        jwt::{generate_access_token, generate_refresh_token},
        password::{hash_password, verify_password},
    },
    db::Db,
    models::{refresh_token::RefreshToken, user::User},
};

pub async fn register(db: &Db, req: RegisterRequest) -> Result<AuthResponse> {
    let hashed = hash_password(&req.password)?;
    let person_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let refresh_id = Uuid::new_v4();
    let verify_id = Uuid::new_v4();
    let mut tx = db.begin().await?;

    sqlx::query(
        "INSERT INTO persons (id, first_name, middle_name, last_name) VALUES ($1,$2,'',$3)",
    )
    .bind(person_id)
    .bind(&req.first_name)
    .bind(&req.last_name)
    .execute(&mut *tx)
    .await?;

    sqlx::query("INSERT INTO person_contacts (id, person_id, email, phone) VALUES ($1,$2,$3,$4)")
        .bind(Uuid::new_v4())
        .bind(person_id)
        .bind(&req.email)
        .bind(req.phone.clone().unwrap_or_default())
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "INSERT INTO users (id, user_name, email, phone, password_hash, person_id) VALUES ($1,$2,$3,$4,$5,$6)",
    )
    .bind(user_id)
    .bind(&req.user_name)
    .bind(&req.email)
    .bind(req.phone.clone().unwrap_or_default())
    .bind(&hashed)
    .bind(person_id)
    .execute(&mut *tx)
    .await?;

    let refresh_token = generate_refresh_token(user_id);
    let expires = (Utc::now() + Duration::days(REFRESH_TOKEN_TTL_DAYS)).naive_utc();

    sqlx::query("INSERT INTO refresh_tokens (id, user_id, token, expires_at) VALUES ($1,$2,$3,$4)")
        .bind(refresh_id)
        .bind(user_id)
        .bind(&refresh_token)
        .bind(expires)
        .execute(&mut *tx)
        .await?;

    let access_token = generate_access_token(user_id);

    let token = generate_verification_token();
    let token_exp = token_expiry_time();

    sqlx::query(
        "INSERT INTO email_verification_tokens (id, user_id, token, expires_at) VALUES ($1,$2,$3,$4)",
    )
    .bind(verify_id)
    .bind(user_id)
    .bind(&token)
    .bind(token_exp)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    send_email_stub(&req.email, &token);

    Ok(AuthResponse {
        access_token,
        refresh_token,
    })
}

pub async fn login(db: &Db, req: LoginRequest) -> Result<AuthResponse> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_name = $1")
        .bind(&req.user_name)
        .fetch_one(db)
        .await?;

    verify_password(&user.password_hash, &req.password)
        .map_err(|e| anyhow!("Password verification failed: {}", e))?;

    let access_token = generate_access_token(user.id);
    let refresh_token = generate_refresh_token(user.id);
    let expires = (Utc::now() + Duration::days(REFRESH_TOKEN_TTL_DAYS)).naive_utc();

    sqlx::query("INSERT INTO refresh_tokens (id, user_id, token, expires_at) VALUES ($1,$2,$3,$4)")
        .bind(Uuid::new_v4())
        .bind(user.id)
        .bind(&refresh_token)
        .bind(expires)
        .execute(db)
        .await?;

    Ok(AuthResponse {
        access_token,
        refresh_token,
    })
}

pub async fn refresh(db: &Db, req: RefreshRequest) -> Result<AuthResponse> {
    let token_row =
        sqlx::query_as::<_, RefreshToken>("SELECT * FROM refresh_tokens WHERE token = $1")
            .bind(&req.refresh_token)
            .fetch_one(db)
            .await?;

    if token_row.expires_at < Utc::now().naive_utc() {
        anyhow::bail!("Token expired");
    }

    let access_token = generate_access_token(token_row.user_id);
    let new_refresh_token = generate_refresh_token(token_row.user_id);
    let expires = (Utc::now() + Duration::days(REFRESH_TOKEN_TTL_DAYS)).naive_utc();

    sqlx::query("UPDATE refresh_tokens SET token = $1, expires_at = $2 WHERE id = $3")
        .bind(&new_refresh_token)
        .bind(expires)
        .bind(token_row.id)
        .execute(db)
        .await?;

    Ok(AuthResponse {
        access_token,
        refresh_token: new_refresh_token,
    })
}

pub async fn forgot_password(db: &Db, req: ForgotPasswordRequest) -> Result<()> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&req.email)
        .fetch_one(db)
        .await?;

    let token = generate_verification_token();
    let token_exp = token_expiry_time();

    sqlx::query(
        "INSERT INTO email_verification_tokens (id, user_id, token, expires_at) VALUES ($1,$2,$3,$4)",
    )
    .bind(Uuid::new_v4())
    .bind(user.id)
    .bind(&token)
    .bind(token_exp)
    .execute(db)
    .await?;

    send_email_stub(&req.email, &token);
    Ok(())
}

pub async fn change_password(db: &Db, user_id: Uuid, req: ChangePasswordRequest) -> Result<()> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(db)
        .await?;

    verify_password(&user.password_hash, &req.current_password)?;
    let new_hash = hash_password(&req.new_password)?;
    sqlx::query("UPDATE users SET password_hash = $1 WHERE id = $2")
        .bind(new_hash)
        .bind(user_id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn get_profile(db: &Db, user_id: Uuid) -> Result<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(db)
        .await
        .map_err(Into::into)
}
