use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{ACCESS_TOKEN_TTL_MINUTES, REFRESH_TOKEN_TTL_DAYS};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

const SECRET: &[u8] = b"your-secret-key";

pub fn generate_access_token(user_id: Uuid) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(ACCESS_TOKEN_TTL_MINUTES))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .unwrap()
}

pub fn generate_refresh_token(user_id: Uuid) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(REFRESH_TOKEN_TTL_DAYS))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .unwrap()
}

pub fn validate_token(token: &str) -> Option<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .ok()
}
