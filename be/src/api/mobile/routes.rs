use axum::{
    Router,
    routing::{get, post, put},
};

use super::handlers;

/// Mounted inside the authenticated router but outside every RBAC layer —
/// see the module docs for why. Each handler re-derives the caller's own
/// employee row, and the two surfaces that genuinely are privileged
/// (leave approvals, document browsing) check authorisation themselves.
pub fn mobile_routes() -> Router {
    Router::new()
        .route("/bootstrap", get(handlers::bootstrap_handler))
        .route("/dashboard", get(handlers::dashboard_handler))
        .route("/attendance/today", get(handlers::today_attendance_handler))
        .route("/attendance/check-in", post(handlers::check_in_handler))
        .route("/attendance/check-out", post(handlers::check_out_handler))
        .route(
            "/attendance/records",
            get(handlers::attendance_records_handler),
        )
        .route("/leave/types", get(handlers::leave_types_handler))
        .route("/leave/balance", get(handlers::leave_balance_handler))
        .route(
            "/leave/requests",
            get(handlers::my_leave_requests_handler)
                .post(handlers::create_leave_request_handler),
        )
        .route("/leave/approvals", get(handlers::leave_approvals_handler))
        .route(
            "/leave/requests/{id}/approve",
            post(handlers::approve_leave_handler),
        )
        .route(
            "/leave/requests/{id}/reject",
            post(handlers::reject_leave_handler),
        )
        .route("/schedule", get(handlers::schedule_handler))
        .route("/profile", put(handlers::update_profile_handler))
        .route("/directory", get(handlers::directory_handler))
        .route("/documents", get(handlers::browse_documents_handler))
        .route("/documents/clients", get(handlers::document_clients_handler))
        .route(
            "/documents/{id}/file",
            get(handlers::download_document_handler),
        )
}
