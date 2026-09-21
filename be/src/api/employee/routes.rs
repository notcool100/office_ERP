use crate::api::employee::handlers;
use crate::middlewares::rbac;
use axum::{
    Router,
    routing::{delete, get, post, put},
};

pub fn employee_routes() -> Router {
    let crud_routes = Router::new()
        .route("/", post(handlers::create_employee_handler))
        .route("/", get(handlers::list_employees_handler))
        .route("/{id}", get(handlers::get_employee_handler))
        .route("/{id}", put(handlers::update_employee_handler))
        .route("/{id}", delete(handlers::delete_employee_handler))
        .layer(rbac::require_permission("/admin/hr/employee"));

    let face_routes = Router::new()
        .route(
            "/config/descriptors",
            get(handlers::list_face_descriptors_handler),
        )
        .route(
            "/{id}/face-descriptor",
            post(handlers::update_face_descriptor_handler),
        )
        .layer(rbac::require_permission(
            "/admin/hr/attendance/register-face",
        ));

    let me_route = Router::new()
        .route("/me", get(handlers::get_my_employee_handler));

    // Authenticated-only (no `/admin/hr/*` RBAC grant required): the
    // attendance kiosk is meant to be usable by any logged-in employee to
    // clock in/out via face recognition, so it must not depend on an
    // HR-admin permission the way `/employees` and `/config/descriptors` do.
    // Both handlers only ever return active employees and carry no PII
    // (no email/phone/salary) beyond what's needed to label a matched face.
    let kiosk_routes = Router::new()
        .route("/kiosk/roster", get(handlers::list_kiosk_employees_handler))
        .route(
            "/kiosk/descriptors",
            get(handlers::list_face_descriptors_handler),
        );

    crud_routes
        .merge(face_routes)
        .merge(me_route)
        .merge(kiosk_routes)
}
