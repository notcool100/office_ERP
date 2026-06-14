use axum::{
    Router,
    routing::{delete, get, post, put},
};

use super::handlers;

pub fn payroll_routes() -> Router {
    Router::new()
        .route("/salary-structures", get(handlers::list_salary_structures_handler))
        .route("/salary-structures", post(handlers::create_salary_structure_handler))
        .route("/salary-structures/{id}", delete(handlers::delete_salary_structure_handler))
        .route("/runs", get(handlers::list_payroll_runs_handler))
        .route("/runs", post(handlers::create_payroll_run_handler))
        .route("/runs/{id}", get(handlers::get_payroll_run_handler))
        .route("/runs/{id}", delete(handlers::delete_payroll_run_handler))
        .route("/runs/{id}/process", post(handlers::process_payroll_run_handler))
        .route("/runs/{id}/pay", post(handlers::mark_run_paid_handler))
        .route("/entries/{id}", put(handlers::update_payroll_entry_handler))
}

/// Separate router — no RBAC, just auth middleware
pub fn payroll_my_routes() -> Router {
    Router::new().route("/my-payslips", get(handlers::my_payslips_handler))
}
