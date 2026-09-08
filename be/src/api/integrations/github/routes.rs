use axum::{
    Router,
    routing::{get, post},
};

use super::handlers;

/// Global GitHub connector, mounted at `/integrations/github`.
pub fn github_connection_routes() -> Router {
    Router::new().route(
        "/",
        get(handlers::get_connection_handler)
            .post(handlers::connect_handler)
            .delete(handlers::disconnect_handler),
    )
}

/// Per-project repo link + sync, mounted at `/projects/{id}/github` from
/// inside `project::routes::project_routes()`.
pub fn project_github_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(handlers::get_project_link_handler)
                .post(handlers::link_project_handler)
                .delete(handlers::unlink_project_handler),
        )
        .route("/sync", post(handlers::sync_project_handler))
}
