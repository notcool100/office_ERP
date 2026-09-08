use chrono::{NaiveDateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::project::service::{GithubIssueSyncInput, upsert_card_from_github_issue},
    models::user::User,
};

use super::{client, crypto};

#[derive(Debug, thiserror::Error)]
pub enum GithubError {
    #[error("{0}")]
    BadRequest(String),
    #[error("Not found")]
    NotFound,
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    Internal(String),
}

#[derive(sqlx::FromRow)]
pub struct GithubConnectionRow {
    pub access_token_encrypted: String,
    pub github_username: String,
    pub connected_at: NaiveDateTime,
}

#[derive(sqlx::FromRow)]
pub struct ProjectGithubLinkRow {
    pub project_id: Uuid,
    pub repo_owner: String,
    pub repo_name: String,
    pub sync_enabled: bool,
    pub last_synced_at: Option<NaiveDateTime>,
    pub last_sync_status: Option<String>,
    pub last_sync_error: Option<String>,
}

pub async fn get_connection(pool: &PgPool) -> Result<Option<GithubConnectionRow>, GithubError> {
    let row = sqlx::query_as::<_, GithubConnectionRow>(
        "SELECT access_token_encrypted, github_username, connected_at FROM github_connections ORDER BY connected_at DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn connect(
    pool: &PgPool,
    user: &User,
    token: String,
) -> Result<GithubConnectionRow, GithubError> {
    let token = token.trim();
    if token.is_empty() {
        return Err(GithubError::BadRequest(
            "GitHub token is required".to_string(),
        ));
    }

    let gh_user = client::validate_token(token)
        .await
        .map_err(|e| GithubError::BadRequest(e.to_string()))?;

    let encrypted =
        crypto::encrypt_token(token).map_err(|e| GithubError::Internal(e.to_string()))?;

    let now = Utc::now().naive_utc();
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM github_connections")
        .execute(&mut *tx)
        .await?;
    let row = sqlx::query_as::<_, GithubConnectionRow>(
        r#"
        INSERT INTO github_connections (access_token_encrypted, github_username, connected_by, connected_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING access_token_encrypted, github_username, connected_at
        "#,
    )
    .bind(&encrypted)
    .bind(&gh_user.login)
    .bind(user.id)
    .bind(now)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;

    Ok(row)
}

pub async fn disconnect(pool: &PgPool) -> Result<(), GithubError> {
    sqlx::query("DELETE FROM github_connections")
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_project_link(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Option<ProjectGithubLinkRow>, GithubError> {
    let row = sqlx::query_as::<_, ProjectGithubLinkRow>(
        r#"
        SELECT project_id, repo_owner, repo_name, sync_enabled,
               last_synced_at, last_sync_status, last_sync_error
        FROM project_github_links
        WHERE project_id = $1
        "#,
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn link_project(
    pool: &PgPool,
    user: &User,
    project_id: Uuid,
    repo_owner: String,
    repo_name: String,
) -> Result<ProjectGithubLinkRow, GithubError> {
    let repo_owner = repo_owner.trim().to_string();
    let repo_name = repo_name.trim().to_string();
    if repo_owner.is_empty() || repo_name.is_empty() {
        return Err(GithubError::BadRequest(
            "Repository owner and name are required".to_string(),
        ));
    }

    let project_exists = sqlx::query_scalar::<_, Uuid>("SELECT id FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_optional(pool)
        .await?;
    if project_exists.is_none() {
        return Err(GithubError::NotFound);
    }

    let connection = get_connection(pool).await?.ok_or_else(|| {
        GithubError::BadRequest(
            "Connect a GitHub account in Settings > Connectors first".to_string(),
        )
    })?;
    let token = crypto::decrypt_token(&connection.access_token_encrypted)
        .map_err(|e| GithubError::Internal(e.to_string()))?;

    client::validate_repo_access(&token, &repo_owner, &repo_name)
        .await
        .map_err(|e| GithubError::BadRequest(e.to_string()))?;

    let now = Utc::now().naive_utc();
    let row = sqlx::query_as::<_, ProjectGithubLinkRow>(
        r#"
        INSERT INTO project_github_links (project_id, repo_owner, repo_name, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $5)
        ON CONFLICT (project_id) DO UPDATE SET
            repo_owner = EXCLUDED.repo_owner,
            repo_name = EXCLUDED.repo_name,
            sync_enabled = true,
            last_synced_at = NULL,
            last_sync_status = NULL,
            last_sync_error = NULL,
            updated_at = EXCLUDED.updated_at
        RETURNING project_id, repo_owner, repo_name, sync_enabled,
                  last_synced_at, last_sync_status, last_sync_error
        "#,
    )
    .bind(project_id)
    .bind(&repo_owner)
    .bind(&repo_name)
    .bind(user.id)
    .bind(now)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn unlink_project(pool: &PgPool, project_id: Uuid) -> Result<(), GithubError> {
    let result = sqlx::query("DELETE FROM project_github_links WHERE project_id = $1")
        .bind(project_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(GithubError::NotFound);
    }

    Ok(())
}

pub struct SyncSummary {
    pub created: i64,
    pub updated: i64,
    pub last_synced_at: NaiveDateTime,
    pub last_sync_status: String,
    pub last_sync_error: Option<String>,
}

/// Fetches issues for a linked project's repo and upserts them as cards,
/// then records the outcome on `project_github_links`.
/// Pushes an ERP-side card move into/out of a "done" column to GitHub as an
/// issue close/reopen. Best-effort: callers run this in the background and
/// just log failures rather than fail the card move itself.
pub async fn push_issue_state(
    pool: &PgPool,
    project_id: Uuid,
    issue_number: i32,
    closed: bool,
) -> Result<(), GithubError> {
    let link = get_project_link(pool, project_id)
        .await?
        .ok_or(GithubError::NotFound)?;

    let connection = get_connection(pool).await?.ok_or_else(|| {
        GithubError::BadRequest(
            "Connect a GitHub account in Settings > Connectors first".to_string(),
        )
    })?;
    let token = crypto::decrypt_token(&connection.access_token_encrypted)
        .map_err(|e| GithubError::Internal(e.to_string()))?;

    let state = if closed { "closed" } else { "open" };
    client::set_issue_state(
        &token,
        &link.repo_owner,
        &link.repo_name,
        issue_number,
        state,
    )
    .await
    .map_err(|e| GithubError::BadRequest(e.to_string()))?;

    Ok(())
}

pub async fn sync_project(pool: &PgPool, project_id: Uuid) -> Result<SyncSummary, GithubError> {
    let link = get_project_link(pool, project_id)
        .await?
        .ok_or(GithubError::NotFound)?;

    let connection = get_connection(pool).await?.ok_or_else(|| {
        GithubError::BadRequest(
            "Connect a GitHub account in Settings > Connectors first".to_string(),
        )
    })?;
    let token = crypto::decrypt_token(&connection.access_token_encrypted)
        .map_err(|e| GithubError::Internal(e.to_string()))?;

    let sync_result =
        sync_with_token(pool, project_id, &link.repo_owner, &link.repo_name, &token).await;

    let now = Utc::now().naive_utc();
    let (created, updated, status, error) = match &sync_result {
        Ok((created, updated)) => (*created, *updated, "success", None),
        Err(e) => (0, 0, "error", Some(e.to_string())),
    };

    sqlx::query(
        r#"
        UPDATE project_github_links
        SET last_synced_at = $1, last_sync_status = $2, last_sync_error = $3, updated_at = $1
        WHERE project_id = $4
        "#,
    )
    .bind(now)
    .bind(status)
    .bind(&error)
    .bind(project_id)
    .execute(pool)
    .await?;

    match sync_result {
        Ok(_) => Ok(SyncSummary {
            created,
            updated,
            last_synced_at: now,
            last_sync_status: status.to_string(),
            last_sync_error: error,
        }),
        Err(e) => Err(GithubError::BadRequest(e.to_string())),
    }
}

async fn sync_with_token(
    pool: &PgPool,
    project_id: Uuid,
    repo_owner: &str,
    repo_name: &str,
    token: &str,
) -> anyhow::Result<(i64, i64)> {
    let issues = client::fetch_issues(token, repo_owner, repo_name).await?;

    let mut created = 0i64;
    let mut updated = 0i64;

    for issue in issues {
        let input = GithubIssueSyncInput {
            number: issue.number,
            title: issue.title,
            description: issue.body,
            state: issue.state,
            html_url: issue.html_url,
        };

        let outcome = upsert_card_from_github_issue(pool, project_id, &input).await?;
        match outcome {
            crate::api::project::service::GithubCardSyncOutcome::Created => created += 1,
            crate::api::project::service::GithubCardSyncOutcome::Updated => updated += 1,
        }
    }

    Ok((created, updated))
}

/// Called by the background scheduler: syncs every project with an enabled
/// GitHub link. Failures are logged per-project and don't abort the batch.
pub async fn sync_all_linked_projects(pool: &PgPool) {
    let project_ids = match sqlx::query_scalar::<_, Uuid>(
        "SELECT project_id FROM project_github_links WHERE sync_enabled = true",
    )
    .fetch_all(pool)
    .await
    {
        Ok(ids) => ids,
        Err(e) => {
            tracing::error!("[GITHUB SYNC] Failed to list linked projects: {}", e);
            return;
        }
    };

    for project_id in project_ids {
        if let Err(e) = sync_project(pool, project_id).await {
            tracing::error!(
                "[GITHUB SYNC] Sync failed for project {}: {}",
                project_id,
                e
            );
        }
    }
}
