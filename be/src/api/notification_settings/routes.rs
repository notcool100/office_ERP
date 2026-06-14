use axum::{Router, routing::get};

use super::handlers::{get_handler, list_handler, update_handler};

pub fn notification_settings_routes() -> Router {
    Router::new()
        .route("/", get(list_handler))
        .route("/:key", get(get_handler).put(update_handler))
}
