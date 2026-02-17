use crate::api::permissions::handlers;
use axum::{
    Router,
    routing::{delete, get, post},
};

pub fn permissions_routes() -> Router {
    Router::new()
        .route("/", post(handlers::assign_permission_handler))
        .route("/", get(handlers::get_permissions_handler))
        .route("/{id}", delete(handlers::delete_permission_handler))
}
