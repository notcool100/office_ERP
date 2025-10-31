use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::models::service_response::ServiceResponse;

pub async fn health_check_handler() -> impl IntoResponse {
    let res = ServiceResponse::<()>::builder()
        .success(true)
        .message("Healthy");
    (StatusCode::OK, Json(res))
}
