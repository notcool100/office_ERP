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
        .route(
            "/{id}/cards/{card_id}/comments",
            get(handlers::list_card_comments_handler),
        )
        .route(
            "/{id}/cards/{card_id}/comments",
            post(handlers::create_card_comment_handler),
        )
        .route(
            "/{id}/cards/{card_id}/attachments",
            get(handlers::list_card_attachments_handler),
        )
        .route(
            "/{id}/cards/{card_id}/attachments",
            post(handlers::upload_card_attachment_handler),
        )
        .route(
            "/{id}/cards/{card_id}/attachments/{attachment_id}",
            get(handlers::download_card_attachment_handler),
        )
        .route(
            "/{id}/cards/{card_id}/history",
            get(handlers::list_card_history_handler),
        )
        .route(
            "/{id}/cards/{card_id}/links",
            get(handlers::list_card_links_handler),
        )
        .route(
            "/{id}/cards/{card_id}/links",
            post(handlers::create_card_link_handler),
        )
        .route(
            "/{id}/cards/{card_id}/links/{link_id}",
            delete(handlers::delete_card_link_handler),
        )
        .route("/{id}/sprints", get(handlers::list_sprints_handler))
        .route("/{id}/sprints", post(handlers::create_sprint_handler))
        .route(
            "/{id}/sprints/{sprint_id}",
            put(handlers::update_sprint_handler),
        )
        .route(
            "/{id}/sprints/{sprint_id}",
            delete(handlers::delete_sprint_handler),
        )
}
