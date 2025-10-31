use crate::api::auth::handlers::{
    change_password_handler, forgot_password_handler, login_handler, profile_handler,
    refresh_handler, register_handler,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn auth_routes() -> Router {
    Router::new()
        .route(
            "/register",
            // post(register_handler).layer(middleware::from_fn(authenticate)),
            post(register_handler),
        )
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_handler))
        .route("/forgot-password", post(forgot_password_handler))
        .route("/change-password", post(change_password_handler))
        .route("/me", get(profile_handler))
}
