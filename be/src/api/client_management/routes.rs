use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::handlers;

pub fn client_management_routes() -> Router {
    Router::new()
        // clients
        .route("/clients",     get(handlers::list_clients_handler))
        .route("/clients",     post(handlers::create_client_handler))
        .route("/clients/{id}", get(handlers::get_client_handler))
        .route("/clients/{id}", put(handlers::update_client_handler))
        .route("/clients/{id}", delete(handlers::delete_client_handler))
        // deliverables
        .route("/deliverables",     get(handlers::list_deliverables_handler))
        .route("/deliverables",     post(handlers::create_deliverable_handler))
        .route("/deliverables/{id}", get(handlers::get_deliverable_handler))
        .route("/deliverables/{id}", put(handlers::update_deliverable_handler))
        .route("/deliverables/{id}", delete(handlers::delete_deliverable_handler))
}
