use crate::api::leave::handlers;
use axum::{routing::{get, post, put}, Router};

pub fn leave_routes() -> Router {
    Router::new()
        .route("/requests", post(handlers::create_leave_request_handler))
        .route("/requests", get(handlers::list_leave_requests_handler))
        .route("/requests/{id}", get(handlers::get_leave_request_handler))
        .route("/requests/{id}/approve", put(handlers::approve_leave_handler))
        .route("/requests/{id}/reject", put(handlers::reject_leave_handler))
        .route("/types", get(handlers::get_leave_types_handler))
        .route("/balance/{employee_id}", get(handlers::get_leave_balance_handler))
}
