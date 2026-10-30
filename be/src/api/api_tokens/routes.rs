use axum::{
    Router,
    routing::{delete, get},
};

use super::handlers;

/// Every user manages their own tokens, so there is no RBAC layer here;
/// handlers scope every query to the caller.
pub fn api_token_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(handlers::list_handler).post(handlers::create_handler),
        )
        .route("/{id}", delete(handlers::revoke_handler))
}
