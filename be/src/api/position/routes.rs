use crate::api::position::handlers;
use axum::{routing::{delete, get, post, put}, Router};

pub fn position_routes() -> Router {
    Router::new()
        .route("/", post(handlers::create_position_handler))
        .route("/", get(handlers::get_positions_handler))
        .route("/{id}", get(handlers::get_position_handler))
        .route("/{id}", put(handlers::update_position_handler))
        .route("/{id}", delete(handlers::delete_position_handler))
}
