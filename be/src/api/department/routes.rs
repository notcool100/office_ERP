use crate::api::department::handlers;
use axum::{routing::{delete, get, post, put}, Router};

pub fn department_routes() -> Router {
    Router::new()
        .route("/", post(handlers::create_department_handler))
        .route("/", get(handlers::get_departments_handler))
        .route("/{id}", get(handlers::get_department_handler))
        .route("/{id}", put(handlers::update_department_handler))
        .route("/{id}", delete(handlers::delete_department_handler))
}
