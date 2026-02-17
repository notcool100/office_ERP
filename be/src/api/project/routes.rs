use crate::api::project::handlers;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn project_routes() -> Router {
    Router::new()
        .route("/", get(handlers::list_projects_handler))
        .route("/", post(handlers::create_project_handler))
        .route("/{id}", get(handlers::get_project_handler))
        .route("/{id}", put(handlers::update_project_handler))
        .route("/{id}/members", get(handlers::list_project_members_handler))
        .route("/{id}/members", post(handlers::add_project_member_handler))
        .route("/{id}/board", get(handlers::get_project_board_handler))
        .route("/{id}/cards", get(handlers::list_cards_handler))
        .route("/{id}/cards", post(handlers::create_card_handler))
        .route("/{id}/cards/{card_id}", put(handlers::update_card_handler))
        .route(
            "/{id}/cards/{card_id}",
            delete(handlers::delete_card_handler),
        )
}
