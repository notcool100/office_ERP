use chrono::{DateTime, Duration, Utc};
use rand::{Rng, distributions::Alphanumeric};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub const TOKEN_PREFIX: &str = "ubk_";

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ApiToken {
    pub id: Uuid,
    pub name: String,
    pub token_prefix: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedApiToken {
    #[serde(flatten)]
    pub token: ApiToken,
    /// Shown once; only its hash is stored.
    pub secret: String,
}

pub fn hash_token(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    name: &str,
    expires_in_days: Option<i64>,
) -> Result<CreatedApiToken, sqlx::Error> {
    let random: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(40)
        .map(char::from)
        .collect();
    let secret = format!("{TOKEN_PREFIX}{random}");
    let expires_at = expires_in_days.map(|d| Utc::now() + Duration::days(d));

    let token = sqlx::query_as::<_, ApiToken>(
        r#"
        INSERT INTO api_tokens (user_id, name, token_hash, token_prefix, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, token_prefix, expires_at, last_used_at, created_at
        "#,
    )
    .bind(user_id)
    .bind(name)
    .bind(hash_token(&secret))
    .bind(&secret[..TOKEN_PREFIX.len() + 4])
    .bind(expires_at)
    .fetch_one(pool)
    .await?;

    Ok(CreatedApiToken { token, secret })
}

pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<ApiToken>, sqlx::Error> {
    sqlx::query_as::<_, ApiToken>(
        r#"
        SELECT id, name, token_prefix, expires_at, last_used_at, created_at
        FROM api_tokens
        WHERE user_id = $1 AND revoked_at IS NULL
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Returns false when the token doesn't exist, isn't the caller's, or is
/// already revoked.
pub async fn revoke(pool: &PgPool, user_id: Uuid, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE api_tokens SET revoked_at = NOW() WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

/// Resolves a presented secret to its owner, recording the use. `None` for
/// unknown, revoked or expired tokens.
pub async fn authenticate(pool: &PgPool, secret: &str) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar::<_, Uuid>(
        r#"
        UPDATE api_tokens SET last_used_at = NOW()
        WHERE token_hash = $1
          AND revoked_at IS NULL
          AND (expires_at IS NULL OR expires_at > NOW())
        RETURNING user_id
        "#,
    )
    .bind(hash_token(secret))
    .fetch_optional(pool)
    .await
}
