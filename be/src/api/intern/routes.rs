use crate::api::intern::handlers;
use axum::{routing::{delete, get, post, put}, Router};

pub fn intern_routes() -> Router {
    Router::new()
        .route("/", post(handlers::create_intern_handler))
        .route("/", get(handlers::list_interns_handler))
        .route("/{id}", get(handlers::get_intern_handler))
        .route("/{id}", put(handlers::update_intern_handler))
        .route("/{id}", delete(handlers::delete_intern_handler))
}
