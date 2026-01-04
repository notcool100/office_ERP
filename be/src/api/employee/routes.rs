use crate::api::employee::handlers;
use axum::{routing::{delete, get, post, put}, Router};

pub fn employee_routes() -> Router {
    Router::new()
        .route("/", post(handlers::create_employee_handler))
        .route("/", get(handlers::list_employees_handler))
        .route("/{id}", get(handlers::get_employee_handler))
        .route("/{id}", put(handlers::update_employee_handler))
        .route("/{id}", delete(handlers::delete_employee_handler))
}
