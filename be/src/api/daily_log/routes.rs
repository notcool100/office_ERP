use crate::api::daily_log::handlers;
use axum::{
    Router,
    routing::{get, put},
};

pub fn daily_log_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(handlers::list_daily_logs).post(handlers::create_daily_log),
        )
        .route(
            "/{id}",
            put(handlers::update_daily_log).delete(handlers::delete_daily_log),
        )
}
