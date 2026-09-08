use axum::{
    Router,
    routing::{get, post, put},
};

use super::handlers;

/// Unauthenticated: the login page and browser tab need these before the
/// user is signed in. Mounted directly on the top-level router.
pub fn company_settings_public_routes() -> Router {
    Router::new()
        .route("/", get(handlers::get_settings_handler))
        .route("/logo", get(handlers::serve_logo_handler))
        .route("/favicon", get(handlers::serve_favicon_handler))
}

/// Admin-only writes, mounted inside the protected router under a distinct
/// path so it never collides with the public GET-only router above.
pub fn company_settings_admin_routes() -> Router {
    Router::new()
        .route("/", put(handlers::update_details_handler))
        .route("/logo", post(handlers::upload_logo_handler))
        .route("/favicon", post(handlers::upload_favicon_handler))
}
