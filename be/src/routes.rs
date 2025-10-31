use axum::{Router, routing::get};

use crate::api::{auth::routes::auth_routes, home::handlers::health_check_handler};

pub fn build_routes() -> Router {
    Router::new()
        .route("/", get(health_check_handler))
        .nest("/auth", auth_routes())
}
