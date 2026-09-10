use axum::{
    Router,
    routing::{get, post},
};

use super::handlers;

/// Every route here is scoped to the caller's own inbox, so it sits behind
/// authentication but deliberately not behind RBAC — a notification about
/// your own leave request isn't an admin surface.
pub fn notifications_routes() -> Router {
    Router::new()
        .route("/", get(handlers::list_handler))
        .route("/unread-count", get(handlers::unread_count_handler))
        .route("/read-all", post(handlers::mark_all_read_handler))
        .route("/{id}/read", post(handlers::mark_read_handler))
        .route(
            "/devices",
            post(handlers::register_device_handler)
                .delete(handlers::unregister_device_handler),
        )
}
