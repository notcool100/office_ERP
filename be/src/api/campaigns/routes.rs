use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::handlers;

pub fn campaign_routes() -> Router {
    Router::new()
        .route("/", get(handlers::list_campaigns_handler))
        .route("/", post(handlers::create_campaign_handler))
        .route("/{id}", get(handlers::get_campaign_handler))
        .route("/{id}", put(handlers::update_campaign_handler))
        .route("/{id}", delete(handlers::delete_campaign_handler))
}
