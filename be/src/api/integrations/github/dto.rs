use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ConnectGithubDto {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct GithubConnectionResponseDto {
    pub connected: bool,
    pub github_username: Option<String>,
    pub connected_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct LinkProjectGithubDto {
    pub repo_owner: String,
    pub repo_name: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectGithubLinkResponseDto {
    pub linked: bool,
    pub repo_owner: Option<String>,
    pub repo_name: Option<String>,
    pub sync_enabled: Option<bool>,
    pub last_synced_at: Option<NaiveDateTime>,
    pub last_sync_status: Option<String>,
    pub last_sync_error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GithubSyncResultDto {
    pub created: i64,
    pub updated: i64,
    pub last_synced_at: NaiveDateTime,
    pub last_sync_status: String,
    pub last_sync_error: Option<String>,
}
