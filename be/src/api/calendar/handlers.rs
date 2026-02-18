use crate::{
    api::calendar::{dto::*, service},
    db::Db,
    models::user::User,
};
use axum::{
    Json,
    extract::{Extension, Path, Query},
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;

fn map_error(err: service::CalendarError) -> StatusCode {
    match err {
        service::CalendarError::NotFound => StatusCode::NOT_FOUND,
        service::CalendarError::Forbidden => StatusCode::FORBIDDEN,
        service::CalendarError::BadRequest(_) => StatusCode::BAD_REQUEST,
        service::CalendarError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn list_events_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Query(query): Query<ListCalendarEventsQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let events = service::list_events(&db, &user, query)
        .await
        .map_err(map_error)?;

    let response: Vec<CalendarEventResponseDto> = events
        .into_iter()
        .map(|event| CalendarEventResponseDto {
            id: event.id,
            title: event.title,
            description: event.description,
            location: event.location,
            start_at: event.start_at,
            end_at: event.end_at,
            all_day: event.all_day,
            scope: event.scope,
            department_id: event.department_id,
            created_by: event.created_by,
            created_at: event.created_at,
            updated_at: event.updated_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn create_event_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateCalendarEventDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let event = service::create_event(&db, &user, payload)
        .await
        .map_err(map_error)?;

    let response = CalendarEventResponseDto {
        id: event.id,
        title: event.title,
        description: event.description,
        location: event.location,
        start_at: event.start_at,
        end_at: event.end_at,
        all_day: event.all_day,
        scope: event.scope,
        department_id: event.department_id,
        created_by: event.created_by,
        created_at: event.created_at,
        updated_at: event.updated_at,
    };

    Ok((StatusCode::CREATED, Json(json!(response))))
}

pub async fn update_event_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCalendarEventDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let event = service::update_event(&db, &user, id, payload)
        .await
        .map_err(map_error)?;

    let response = CalendarEventResponseDto {
        id: event.id,
        title: event.title,
        description: event.description,
        location: event.location,
        start_at: event.start_at,
        end_at: event.end_at,
        all_day: event.all_day,
        scope: event.scope,
        department_id: event.department_id,
        created_by: event.created_by,
        created_at: event.created_at,
        updated_at: event.updated_at,
    };

    Ok((StatusCode::OK, Json(json!(response))))
}

pub async fn delete_event_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::delete_event(&db, &user, id)
        .await
        .map_err(map_error)?;

    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Event deleted successfully" })),
    ))
}
