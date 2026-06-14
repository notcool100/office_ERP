use crate::api::content_calendar::handlers;
use axum::{Router, routing::get};

pub fn content_calendar_routes() -> Router {
    Router::new()
        .route("/", get(handlers::list_items_handler).post(handlers::create_item_handler))
        .route("/{id}", get(handlers::get_item_handler)
            .put(handlers::update_item_handler)
            .delete(handlers::delete_item_handler))
}
