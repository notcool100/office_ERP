use axum::{routing::{get, post, put, delete}, Router, middleware};
use crate::api::person::handlers;
use crate::middlewares::auth::authenticate;

pub fn person_routes() -> Router {
    Router::new()
        .route("/", post(handlers::create_person_handler))
        .route("/", get(handlers::list_persons_handler))
        .route("/{id}", get(handlers::get_person_handler))
        .route("/{id}", put(handlers::update_person_handler))
        .route("/{id}", delete(handlers::delete_person_handler))
        .layer(middleware::from_fn(authenticate)) // Protect all person routes
}
