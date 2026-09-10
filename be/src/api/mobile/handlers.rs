use axum::{
    Json,
    body::Body,
    extract::{Extension, Path, Query},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    api::documents::{dto::DocCategory, service as documents_service},
    db::Db,
    middlewares::rbac,
    models::user::User,
    ws::hub::Hub,
};

use super::{
    dto::{
        DateRangeQuery, DocumentBrowseQuery, LeaveDecisionRequest, MobileCheckInRequest,
        MobileCheckOutRequest, MobileCreateLeaveRequest, UpdateProfileRequest,
    },
    service::{self, LeaveDecision},
};

/// A failure the user can act on (out of geofence, already checked in,
/// no employee record) versus one they can't. Everything in this module
/// funnels through here so the app can show the message verbatim instead
/// of a generic "something went wrong".
fn user_error(e: anyhow::Error) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "message": e.to_string() })),
    )
        .into_response()
}

fn server_error(e: anyhow::Error) -> Response {
    tracing::error!("mobile: {}", e);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "message": "Something went wrong" })),
    )
        .into_response()
}

fn ok<T: serde::Serialize>(value: T) -> Response {
    (StatusCode::OK, Json(json!(value))).into_response()
}

pub async fn bootstrap_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Response {
    match service::bootstrap(&db, &user).await {
        Ok(payload) => ok(payload),
        Err(e) => server_error(e),
    }
}

pub async fn dashboard_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Response {
    match service::dashboard(&db, &user).await {
        Ok(payload) => ok(payload),
        Err(e) => server_error(e),
    }
}

// -------------------------------------------------------------- attendance

pub async fn check_in_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<MobileCheckInRequest>,
) -> Response {
    match service::check_in(&db, &user, payload).await {
        Ok(record) => (StatusCode::CREATED, Json(json!(record))).into_response(),
        Err(e) => user_error(e),
    }
}

pub async fn check_out_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<MobileCheckOutRequest>,
) -> Response {
    match service::check_out(&db, &user, payload).await {
        Ok(record) => ok(record),
        Err(e) => user_error(e),
    }
}

pub async fn today_attendance_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Response {
    let employee = match service::resolve_employee(&db, &user).await {
        Ok(e) => e,
        Err(e) => return user_error(e),
    };

    match service::today_attendance(&db, employee.id, service::today_in_nepal()).await {
        Ok(record) => ok(record),
        Err(e) => server_error(e),
    }
}

pub async fn attendance_records_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Query(range): Query<DateRangeQuery>,
) -> Response {
    match service::attendance_records(&db, &user, range).await {
        Ok(records) => ok(records),
        Err(e) => user_error(e),
    }
}

// ------------------------------------------------------------------- leave

pub async fn leave_types_handler(Extension(db): Extension<Db>) -> Response {
    match service::leave_types(&db).await {
        Ok(types) => ok(types),
        Err(e) => server_error(e),
    }
}

pub async fn leave_balance_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Response {
    match service::leave_balance(&db, &user).await {
        Ok(balances) => ok(balances),
        Err(e) => user_error(e),
    }
}

pub async fn my_leave_requests_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Response {
    match service::my_leave_requests(&db, &user).await {
        Ok(requests) => ok(requests),
        Err(e) => user_error(e),
    }
}

pub async fn create_leave_request_handler(
    Extension(db): Extension<Db>,
    Extension(hub): Extension<Arc<Hub>>,
    Extension(user): Extension<User>,
    Json(payload): Json<MobileCreateLeaveRequest>,
) -> Response {
    match service::create_leave_request(&db, Some(&hub), &user, payload).await {
        Ok(request) => (StatusCode::CREATED, Json(json!(request))).into_response(),
        Err(e) => user_error(e),
    }
}

pub async fn leave_approvals_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Response {
    let employee_id = match service::resolve_employee(&db, &user).await {
        Ok(e) => e.id,
        // An admin without an employee record still approves leave.
        Err(_) if user.is_admin => Uuid::nil(),
        Err(e) => return user_error(e),
    };

    match service::pending_approvals(&db, &user, employee_id).await {
        Ok(requests) => ok(requests),
        Err(e) => server_error(e),
    }
}

pub async fn approve_leave_handler(
    Extension(db): Extension<Db>,
    Extension(hub): Extension<Arc<Hub>>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<LeaveDecisionRequest>,
) -> Response {
    decide(db, hub, user, id, LeaveDecision::Approve, payload).await
}

pub async fn reject_leave_handler(
    Extension(db): Extension<Db>,
    Extension(hub): Extension<Arc<Hub>>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<LeaveDecisionRequest>,
) -> Response {
    decide(db, hub, user, id, LeaveDecision::Reject, payload).await
}

async fn decide(
    db: Db,
    hub: Arc<Hub>,
    user: User,
    id: Uuid,
    decision: LeaveDecision,
    payload: LeaveDecisionRequest,
) -> Response {
    match service::decide_leave(&db, Some(&hub), &user, id, decision, payload).await {
        Ok(request) => ok(request),
        Err(e) => (
            StatusCode::FORBIDDEN,
            Json(json!({ "message": e.to_string() })),
        )
            .into_response(),
    }
}

// ------------------------------------------------------- profile & roster

pub async fn schedule_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Response {
    match service::schedule(&db, &user).await {
        Ok(days) => ok(days),
        Err(e) => user_error(e),
    }
}

pub async fn update_profile_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Response {
    match service::update_profile(&db, &user, payload).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => user_error(e),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryQuery {
    pub search: Option<String>,
}

pub async fn directory_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Query(query): Query<DirectoryQuery>,
) -> Response {
    match service::directory(&db, &user, query.search).await {
        Ok(entries) => ok(entries),
        Err(e) => server_error(e),
    }
}

// --------------------------------------------------------------- documents

/// Read-only document browsing. The category comes from the query string
/// here rather than from the mount point, so the permission for that
/// category has to be re-checked on every call — same grant the admin
/// router enforces, just returning 403 instead of routing around it.
pub async fn browse_documents_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Query(query): Query<DocumentBrowseQuery>,
) -> Response {
    let category = service::parse_category(query.category.as_deref());

    if !rbac::user_can_read(&db, &user, service::nav_path_for(category)).await {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "message": "You do not have access to these documents" })),
        )
            .into_response();
    }

    match documents_service::list_location(&db, category, query.owner_id, query.folder_id).await {
        Ok(listing) => ok(listing),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn document_clients_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Response {
    if !rbac::user_can_read(&db, &user, service::nav_path_for(DocCategory::Client)).await {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "message": "You do not have access to client documents" })),
        )
            .into_response();
    }

    match service::list_document_clients(&db).await {
        Ok(clients) => ok(clients),
        Err(e) => server_error(e),
    }
}

pub async fn download_document_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Query(query): Query<DocumentBrowseQuery>,
) -> Response {
    let category = service::parse_category(query.category.as_deref());

    if !rbac::user_can_read(&db, &user, service::nav_path_for(category)).await {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "message": "You do not have access to these documents" })),
        )
            .into_response();
    }

    let doc_file =
        match documents_service::get_document_file(&db, category, query.owner_id, id).await {
            Ok(f) => f,
            Err(_) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "message": "Document not found" })),
                )
                    .into_response();
            }
        };

    let file = match File::open(&doc_file.file_path).await {
        Ok(f) => f,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "message": "File not found on disk" })),
            )
                .into_response();
        }
    };

    let disposition = format!("inline; filename=\"{}\"", doc_file.file_name);
    (
        [
            (header::CONTENT_TYPE, doc_file.content_type),
            (header::CONTENT_DISPOSITION, disposition),
        ],
        Body::from_stream(ReaderStream::new(file)),
    )
        .into_response()
}
