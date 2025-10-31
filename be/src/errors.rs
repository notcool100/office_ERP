use axum::{http::StatusCode, response::IntoResponse};

pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self.message).into_response()
    }
}
