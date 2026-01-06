use crate::api::auth::handlers::{
    change_password_handler, forgot_password_handler, login_handler, profile_handler,
    refresh_handler, register_handler,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn auth_routes() -> Router {
    let public_routes = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/refresh", post(refresh_handler))
        .route("/forgot-password", post(forgot_password_handler));

    let protected_routes = Router::new()
        .route("/change-password", post(change_password_handler))
        .route("/me", get(profile_handler))
        .route_layer(axum::middleware::from_fn(
            crate::middlewares::auth::authenticate,
        ));

    public_routes.merge(protected_routes)
}
