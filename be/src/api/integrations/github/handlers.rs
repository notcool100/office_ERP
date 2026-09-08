use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{db::Db, models::user::User};

use super::{
    dto::{
        ConnectGithubDto, GithubConnectionResponseDto, GithubSyncResultDto, LinkProjectGithubDto,
        ProjectGithubLinkResponseDto,
    },
    service::{self, GithubError},
};

fn error_response(err: GithubError) -> (StatusCode, Json<serde_json::Value>) {
    let status = match err {
        GithubError::BadRequest(_) => StatusCode::BAD_REQUEST,
        GithubError::NotFound => StatusCode::NOT_FOUND,
        GithubError::Database(ref e) => {
            tracing::error!("[github] database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
        GithubError::Internal(ref msg) => {
            tracing::error!("[github] internal error: {}", msg);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    };

    (
        status,
        Json(serde_json::json!({ "message": err.to_string() })),
    )
}

pub async fn get_connection_handler(Extension(db): Extension<Db>) -> impl IntoResponse {
    match service::get_connection(&db).await {
        Ok(Some(conn)) => (
            StatusCode::OK,
            Json(serde_json::json!(GithubConnectionResponseDto {
                connected: true,
                github_username: Some(conn.github_username),
                connected_at: Some(conn.connected_at),
            })),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::OK,
            Json(serde_json::json!(GithubConnectionResponseDto {
                connected: false,
                github_username: None,
                connected_at: None,
            })),
        )
            .into_response(),
        Err(e) => error_response(e).into_response(),
    }
}

pub async fn connect_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<ConnectGithubDto>,
) -> impl IntoResponse {
    match service::connect(&db, &user, payload.token).await {
        Ok(conn) => (
            StatusCode::OK,
            Json(serde_json::json!(GithubConnectionResponseDto {
                connected: true,
                github_username: Some(conn.github_username),
                connected_at: Some(conn.connected_at),
            })),
        )
            .into_response(),
        Err(e) => error_response(e).into_response(),
    }
}

pub async fn disconnect_handler(Extension(db): Extension<Db>) -> impl IntoResponse {
    match service::disconnect(&db).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => error_response(e).into_response(),
    }
}

fn link_to_dto(link: Option<service::ProjectGithubLinkRow>) -> ProjectGithubLinkResponseDto {
    match link {
        Some(link) => ProjectGithubLinkResponseDto {
            linked: true,
            repo_owner: Some(link.repo_owner),
            repo_name: Some(link.repo_name),
            sync_enabled: Some(link.sync_enabled),
            last_synced_at: link.last_synced_at,
            last_sync_status: link.last_sync_status,
            last_sync_error: link.last_sync_error,
        },
        None => ProjectGithubLinkResponseDto {
            linked: false,
            repo_owner: None,
            repo_name: None,
            sync_enabled: None,
            last_synced_at: None,
            last_sync_status: None,
            last_sync_error: None,
        },
    }
}

pub async fn get_project_link_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service::get_project_link(&db, id).await {
        Ok(link) => (StatusCode::OK, Json(serde_json::json!(link_to_dto(link)))).into_response(),
        Err(e) => error_response(e).into_response(),
    }
}

pub async fn link_project_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<LinkProjectGithubDto>,
) -> impl IntoResponse {
    match service::link_project(&db, &user, id, payload.repo_owner, payload.repo_name).await {
        Ok(link) => (
            StatusCode::OK,
            Json(serde_json::json!(link_to_dto(Some(link)))),
        )
            .into_response(),
        Err(e) => error_response(e).into_response(),
    }
}

pub async fn unlink_project_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service::unlink_project(&db, id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => error_response(e).into_response(),
    }
}

pub async fn sync_project_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service::sync_project(&db, id).await {
        Ok(summary) => (
            StatusCode::OK,
            Json(serde_json::json!(GithubSyncResultDto {
                created: summary.created,
                updated: summary.updated,
                last_synced_at: summary.last_synced_at,
                last_sync_status: summary.last_sync_status,
                last_sync_error: summary.last_sync_error,
            })),
        )
            .into_response(),
        Err(e) => error_response(e).into_response(),
    }
}
