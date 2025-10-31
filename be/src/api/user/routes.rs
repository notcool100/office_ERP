use axum::{Router, routing::get};

use crate::api::user::handler::list_users_handler;

pub fn user_routes() -> Router {
    Router::new().route("/", get(list_users_handler))
}
