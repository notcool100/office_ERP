use crate::api::person::handlers;
use crate::middlewares::rbac;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn person_routes() -> Router {
    let crud_routes = Router::new()
        .route("/", post(handlers::create_person_handler))
        .route("/", get(handlers::list_persons_handler))
        .route("/{id}", get(handlers::get_person_handler))
        .route("/{id}", put(handlers::update_person_handler))
        .route("/{id}", delete(handlers::delete_person_handler))
        .layer(rbac::require_permission("/admin/hr/person"));

    let me_route = Router::new()
        .route("/me", get(handlers::get_my_person_handler));

    crud_routes.merge(me_route)
}
