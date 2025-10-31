use axum::{Router, routing::get};

use crate::api::home::handlers::health_check_handler;

pub fn home_routes() -> Router {
    Router::new().route("/", get(health_check_handler))
}
