use anyhow::{Result, anyhow};
use serde_json::json;

/// Config for talking to a Mailcow instance's REST API to provision real
/// mailboxes alongside ERP user accounts. See https://mail.<domain>/api/
#[derive(Clone)]
pub struct MailcowConfig {
    /// e.g. https://mail.adyatech.com.np:18443
    pub base_url: String,
    pub api_key: String,
    /// Only emails on this domain get a mailbox provisioned.
    pub domain: String,
}

impl MailcowConfig {
    pub fn from_env() -> Option<Self> {
        let base_url = std::env::var("MAILCOW_API_URL").ok()?;
        let api_key = std::env::var("MAILCOW_API_KEY").ok()?;
        let domain =
            std::env::var("MAILCOW_MAIL_DOMAIN").unwrap_or_else(|_| "adyatech.com.np".to_string());
        Some(Self {
            base_url,
            api_key,
            domain,
        })
    }
}

pub struct VmailService;

impl VmailService {
    pub async fn create_mailbox(
        cfg: &MailcowConfig,
        email: &str,
        password: &str,
        name: &str,
    ) -> Result<()> {
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid email address"));
        }
        let local_part = parts[0];
        let domain = parts[1];

        let url = format!(
            "{}/api/v1/add/mailbox",
            cfg.base_url.trim_end_matches('/')
        );

        let body = json!({
            "local_part": local_part,
            "domain": domain,
            "name": name,
            "password": password,
            "password2": password,
            "quota": "3072",
            "active": "1"
        });

        // Mailcow's admin API listener uses a self-signed cert by design
        // (real TLS termination for the public domain happens elsewhere);
        // this call stays host-internal, so we trust it explicitly.
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| anyhow!("Failed to build HTTP client: {}", e))?;
        let resp = client
            .post(&url)
            .header("X-API-Key", &cfg.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("Mailcow API request failed: {}", e))?;

        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(anyhow!("Mailcow API returned {}: {}", status, text));
        }

        // Mailcow returns HTTP 200 with a JSON array of {"type": "success"|"danger"|"error", "msg": [...]}
        // even on failure, so the status field itself has to be inspected.
        if text.contains("\"danger\"") || text.contains("\"error\"") {
            return Err(anyhow!("Mailcow API rejected mailbox creation: {}", text));
        }

        Ok(())
    }
}
