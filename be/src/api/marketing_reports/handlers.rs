use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use super::{dto::ReportQuery, service};
use crate::{db::Db, models::user::User};

pub async fn get_report_handler(
    Extension(db): Extension<Db>,
    Extension(_user): Extension<User>,
    Query(q): Query<ReportQuery>,
) -> Response {
    match service::generate_report(&db, q).await {
        Ok(report) => Json(report).into_response(),
        Err(e) => {
            tracing::error!("Marketing report error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate report").into_response()
        }
    }
}
