use axum::{routing::get, Router};
use super::handlers;

pub fn marketing_report_routes() -> Router {
    Router::new().route("/", get(handlers::get_report_handler))
}
