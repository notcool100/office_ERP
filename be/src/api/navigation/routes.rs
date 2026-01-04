use crate::api::navigation::handlers;
use crate::middlewares::auth::authenticate;
use axum::{
    routing::{delete, get, post, put},
    Router,
    middleware as axum_middleware,
};

pub fn navigation_routes() -> Router {
    Router::new()
        .route("/user", get(handlers::get_user_navigation_handler)
            .layer(axum_middleware::from_fn(authenticate)))
        .route("/", post(handlers::create_navigation_handler))
        .route("/", get(handlers::get_navigation_items_handler))
        .route("/{id}", get(handlers::get_navigation_handler))
        .route("/{id}", put(handlers::update_navigation_handler))
        .route("/{id}", delete(handlers::delete_navigation_handler))
}
