use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::handlers;

pub fn media_library_routes() -> Router {
    Router::new()
        .route("/", get(handlers::list_assets_handler))
        .route("/", post(handlers::upload_asset_handler))
        .route("/{id}", get(handlers::get_asset_handler))
        .route("/{id}", put(handlers::update_asset_handler))
        .route("/{id}", delete(handlers::delete_asset_handler))
        .route("/{id}/file", get(handlers::serve_asset_handler))
}
