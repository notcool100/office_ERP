use crate::{api::content_calendar::{dto::*, service}, db::Db, models::user::User};
use axum::{Json, extract::{Extension, Path, Query}, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

fn map_error(err: service::ContentCalendarError) -> StatusCode {
    match err {
        service::ContentCalendarError::NotFound    => StatusCode::NOT_FOUND,
        service::ContentCalendarError::Forbidden   => StatusCode::FORBIDDEN,
        service::ContentCalendarError::BadRequest(_) => StatusCode::BAD_REQUEST,
        service::ContentCalendarError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn list_items_handler(
    Extension(db): Extension<Db>,
    Query(query): Query<ListContentItemsQuery>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let items = service::list_items(&db, query).await.map_err(map_error)?;
    Ok((StatusCode::OK, Json(json!(items))))
}

pub async fn get_item_handler(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let item = service::get_item(&db, id).await.map_err(map_error)?;
    Ok((StatusCode::OK, Json(json!(item))))
}

pub async fn create_item_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateContentItemDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let item = service::create_item(&db, &user, payload).await.map_err(map_error)?;
    Ok((StatusCode::CREATED, Json(json!(item))))
}

pub async fn update_item_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateContentItemDto>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let item = service::update_item(&db, &user, id, payload).await.map_err(map_error)?;
    Ok((StatusCode::OK, Json(json!(item))))
}

pub async fn delete_item_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    service::delete_item(&db, &user, id).await.map_err(map_error)?;
    Ok((StatusCode::OK, Json(json!({"message": "Item deleted"}))))
}
