use axum::{Router, routing::{get, post}};

use crate::api::user::handler::{create_user_handler, list_users_handler};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(list_users_handler).post(create_user_handler))
}
