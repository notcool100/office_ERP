use crate::api::calendar::handlers;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn calendar_routes() -> Router {
    Router::new()
        .route("/events", get(handlers::list_events_handler))
        .route("/events", post(handlers::create_event_handler))
        .route("/events/{id}", put(handlers::update_event_handler))
        .route("/events/{id}", delete(handlers::delete_event_handler))
}
