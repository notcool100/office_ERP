use axum::{Router, routing::get};

use crate::api::{
    attendance::routes::attendance_routes, auth::routes::auth_routes,
    department::routes::department_routes, employee::routes::employee_routes,
    home::handlers::health_check_handler, intern::routes::intern_routes,
    leave::routes::leave_routes, navigation::routes::navigation_routes,
    permissions::routes::permissions_routes, person::routes::person_routes,
    position::routes::position_routes, project::routes::project_routes, user::routes::user_routes,
};
use crate::middlewares::rbac;

pub fn build_routes() -> Router {
    let protected_routes = Router::new()
        .nest("/employees", employee_routes())
        .nest(
            "/interns",
            intern_routes().layer(rbac::require_permission("/admin/hr/intern")),
        )
        .nest(
            "/leave",
            leave_routes().layer(rbac::require_permission("/admin/hr/leave")),
        )
        .nest(
            "/attendance",
            attendance_routes().layer(rbac::require_permission("/admin/hr/attendance")),
        )
        .nest(
            "/departments",
            department_routes().layer(rbac::require_permission("/admin/settings/department")),
        )
        .nest(
            "/positions",
            position_routes().layer(rbac::require_permission("/admin/settings/position")),
        )
        .nest(
            "/projects",
            project_routes().layer(rbac::require_permission("/admin/projects")),
        )
        .nest("/navigation", navigation_routes())
        .nest(
            "/permissions",
            permissions_routes().layer(rbac::require_permission("/admin/settings/permissions")),
        )
        .nest(
            "/persons",
            person_routes().layer(rbac::require_permission("/admin/hr/person")),
        )
        .nest(
            "/users",
            user_routes().layer(rbac::require_permission("/admin/settings/user")),
        )
        .route_layer(axum::middleware::from_fn(
            crate::middlewares::auth::authenticate,
        ));

    Router::new()
        .route("/", get(health_check_handler))
        .nest("/auth", auth_routes())
        .merge(protected_routes)
}
