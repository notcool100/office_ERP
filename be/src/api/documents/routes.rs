use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};

use super::{dto::DocCategory, handlers};

/// Builds the folders+files endpoints for one document surface. Mounted
/// three times in routes.rs — once per category — each under its own RBAC
/// permission check, with the category baked in via `Extension` so it can't
/// be overridden by request input.
pub fn documents_routes_for(category: DocCategory) -> Router {
    Router::new()
        .route("/", get(handlers::list_location_handler))
        .route("/", post(handlers::upload_document_handler))
        .route("/folders", post(handlers::create_folder_handler))
        .route("/folders/{id}", delete(handlers::delete_folder_handler))
        .route("/{id}", get(handlers::get_document_handler))
        .route("/{id}", put(handlers::update_document_handler))
        .route("/{id}", delete(handlers::delete_document_handler))
        .route("/{id}/file", get(handlers::serve_document_handler))
        .layer(Extension(category))
}
