use crate::api::attendance::handlers;
use axum::{routing::{get, post}, Router};

pub fn attendance_routes() -> Router {
    Router::new()
        .route("/check-in", post(handlers::check_in_handler))
        .route("/check-out/{employee_id}", post(handlers::check_out_handler))
        .route("/records", get(handlers::list_attendance_handler))
        .route("/summary/{employee_id}/{start_date}/{end_date}", get(handlers::get_attendance_summary_handler))
}
