use chrono::{Duration, NaiveDateTime, Utc};
use uuid::Uuid;

use crate::VERIFICATION_TOKEN_TTL_HOURS;

pub fn generate_verification_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn token_expiry_time() -> NaiveDateTime {
    (Utc::now() + Duration::hours(VERIFICATION_TOKEN_TTL_HOURS)).naive_utc()
}

pub fn send_email_stub(to: &str, token: &str) {
    tracing::info!("[EMAIL MOCK] sending token {} to {}", to, token);
}
