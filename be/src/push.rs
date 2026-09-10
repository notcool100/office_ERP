//! Firebase Cloud Messaging sender for the mobile app.
//!
//! Configured entirely from the environment and inert when unconfigured —
//! a deployment without Firebase credentials still stores notifications and
//! pushes them over the WebSocket, it just doesn't wake sleeping phones. So
//! every failure here is logged and swallowed: a push that can't be sent
//! must never fail the request that triggered it.
//!
//! Env:
//!   FCM_PROJECT_ID              Firebase project id
//!   FCM_SERVICE_ACCOUNT_PATH    path to the service-account JSON, or
//!   FCM_SERVICE_ACCOUNT_JSON    the JSON itself (handy for container envs)

use std::sync::OnceLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::RwLock;

const FCM_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";
/// Google's tokens last an hour; refresh a little early so an in-flight
/// send never races the expiry.
const TOKEN_REFRESH_MARGIN_SECS: u64 = 120;

#[derive(Debug, Clone, Deserialize)]
struct ServiceAccount {
    project_id: Option<String>,
    client_email: String,
    private_key: String,
    #[serde(default = "default_token_uri")]
    token_uri: String,
}

fn default_token_uri() -> String {
    "https://oauth2.googleapis.com/token".to_string()
}

#[derive(Debug, Serialize)]
struct JwtClaims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    iat: u64,
    exp: u64,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

pub struct PushConfig {
    project_id: String,
    service_account: ServiceAccount,
    http: reqwest::Client,
    /// Cached OAuth token as (token, unix expiry).
    cached_token: RwLock<Option<(String, u64)>>,
}

/// What a push carries. `data` is echoed back to the app so a tap can be
/// routed to the right screen without another round trip.
pub struct PushMessage {
    pub title: String,
    pub body: String,
    pub data: Value,
}

static CONFIG: OnceLock<Option<PushConfig>> = OnceLock::new();

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn load_service_account() -> Option<ServiceAccount> {
    let raw = if let Ok(inline) = std::env::var("FCM_SERVICE_ACCOUNT_JSON") {
        inline
    } else if let Ok(path) = std::env::var("FCM_SERVICE_ACCOUNT_PATH") {
        match std::fs::read_to_string(&path) {
            Ok(contents) => contents,
            Err(e) => {
                tracing::warn!("FCM_SERVICE_ACCOUNT_PATH {} is unreadable: {}", path, e);
                return None;
            }
        }
    } else {
        return None;
    };

    match serde_json::from_str::<ServiceAccount>(&raw) {
        Ok(account) => Some(account),
        Err(e) => {
            tracing::warn!("FCM service account JSON is malformed: {}", e);
            None
        }
    }
}

fn config() -> Option<&'static PushConfig> {
    CONFIG
        .get_or_init(|| {
            let service_account = load_service_account()?;
            let project_id = std::env::var("FCM_PROJECT_ID")
                .ok()
                .or_else(|| service_account.project_id.clone())?;

            tracing::info!("FCM push enabled for project {}", project_id);
            Some(PushConfig {
                project_id,
                service_account,
                http: reqwest::Client::new(),
                cached_token: RwLock::new(None),
            })
        })
        .as_ref()
}

/// Whether push is configured. Callers use this only to skip work — an
/// unconfigured deployment is a supported state, not an error.
pub fn is_enabled() -> bool {
    config().is_some()
}

impl PushConfig {
    async fn access_token(&self) -> anyhow::Result<String> {
        if let Some((token, expires_at)) = self.cached_token.read().await.as_ref() {
            if *expires_at > now_secs() + TOKEN_REFRESH_MARGIN_SECS {
                return Ok(token.clone());
            }
        }

        let mut guard = self.cached_token.write().await;
        // Another task may have refreshed while we waited for the lock.
        if let Some((token, expires_at)) = guard.as_ref() {
            if *expires_at > now_secs() + TOKEN_REFRESH_MARGIN_SECS {
                return Ok(token.clone());
            }
        }

        let issued_at = now_secs();
        let claims = JwtClaims {
            iss: &self.service_account.client_email,
            scope: FCM_SCOPE,
            aud: &self.service_account.token_uri,
            iat: issued_at,
            exp: issued_at + 3600,
        };
        let key = EncodingKey::from_rsa_pem(self.service_account.private_key.as_bytes())?;
        let assertion = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &key)?;

        let response = self
            .http
            .post(&self.service_account.token_uri)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &assertion),
            ])
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Google token exchange failed ({}): {}", status, body);
        }

        let token: TokenResponse = response.json().await?;
        let expires_at = now_secs() + token.expires_in;
        *guard = Some((token.access_token.clone(), expires_at));
        Ok(token.access_token)
    }
}

/// Outcome of a single send, so the caller knows whether to drop the token.
pub enum SendOutcome {
    Delivered,
    /// FCM says this registration token no longer exists — the row should
    /// be deleted so we stop retrying it forever.
    TokenInvalid,
    Failed,
}

/// Sends one message to one device token.
pub async fn send_to_token(token: &str, message: &PushMessage) -> SendOutcome {
    let Some(cfg) = config() else {
        return SendOutcome::Failed;
    };

    let access_token = match cfg.access_token().await {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!("FCM: could not obtain access token: {}", e);
            return SendOutcome::Failed;
        }
    };

    // FCM data values must all be strings, so stringify anything that
    // isn't already one rather than letting the whole send 400.
    let data = match &message.data {
        Value::Object(map) => map
            .iter()
            .map(|(k, v)| {
                let s = match v {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                };
                (k.clone(), Value::String(s))
            })
            .collect::<serde_json::Map<_, _>>(),
        _ => serde_json::Map::new(),
    };

    let payload = json!({
        "message": {
            "token": token,
            "notification": { "title": message.title, "body": message.body },
            "data": data,
            "android": {
                "priority": "HIGH",
                "notification": { "channel_id": "adya_default", "sound": "default" }
            },
            "apns": {
                "headers": { "apns-priority": "10" },
                "payload": { "aps": { "sound": "default", "badge": 1 } }
            }
        }
    });

    let url = format!(
        "https://fcm.googleapis.com/v1/projects/{}/messages:send",
        cfg.project_id
    );

    let response = cfg
        .http
        .post(&url)
        .bearer_auth(access_token)
        .json(&payload)
        .timeout(Duration::from_secs(10))
        .send()
        .await;

    match response {
        Ok(res) if res.status().is_success() => SendOutcome::Delivered,
        Ok(res) => {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            // 404 UNREGISTERED / 400 INVALID_ARGUMENT on the token both
            // mean the registration is dead.
            if status == reqwest::StatusCode::NOT_FOUND || body.contains("UNREGISTERED") {
                SendOutcome::TokenInvalid
            } else {
                tracing::warn!("FCM send failed ({}): {}", status, body);
                SendOutcome::Failed
            }
        }
        Err(e) => {
            tracing::warn!("FCM send failed: {}", e);
            SendOutcome::Failed
        }
    }
}
