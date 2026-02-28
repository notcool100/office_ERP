use crate::{
    api::messaging::{dto::{CreateChannelRequest, SendMessageRequest, AddMemberRequest, UpdateChannelRequest}, service},
    db::Db,
    models::user::User,
    ws::hub::{Hub, WsMessage},
};
use axum::{Extension, Json, http::StatusCode, extract::{Path, State}};
use std::sync::Arc;
use uuid::Uuid;

pub async fn add_member_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(channel_id): Path<Uuid>,
    Json(payload): Json<AddMemberRequest>,
) -> Result<StatusCode, StatusCode> {
    service::add_member(&db, channel_id, user.id, payload.user_id)
        .await
        .map_err(|e| {
            eprintln!("Error adding member: {}", e);
            StatusCode::BAD_REQUEST
        })?;
    Ok(StatusCode::OK)
}

pub async fn get_channel_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(channel_id): Path<Uuid>,
) -> Result<(StatusCode, Json<crate::models::messaging::Channel>), StatusCode> {
    let channel = service::get_channel(&db, channel_id, user.id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(channel)))
}

pub async fn list_channels_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Result<(StatusCode, Json<Vec<crate::models::messaging::Channel>>), StatusCode> {
    let channels = service::list_channels(&db, user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(channels)))
}

pub async fn create_channel_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>, // Assuming authentication middleware provides the user
    Json(payload): Json<CreateChannelRequest>,
) -> Result<(StatusCode, Json<crate::models::messaging::Channel>), StatusCode> {
    let channel = service::create_channel(&db, payload, user.id)
        .await
        .map_err(|e| {
            eprintln!("Error creating channel: {}", e);
            StatusCode::BAD_REQUEST
        })?;
    Ok((StatusCode::CREATED, Json(channel)))
}

pub async fn list_messages_handler(
    Extension(db): Extension<Db>,
    Path(channel_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<crate::api::messaging::dto::MessageResponse>>), StatusCode> {
    let messages = service::list_messages(&db, channel_id, 50)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(messages)))
}

pub async fn send_message_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    State(hub): State<Arc<Hub>>,
    Path(channel_id): Path<Uuid>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<(StatusCode, Json<crate::models::messaging::Message>), StatusCode> {
    let message = service::send_message(&db, channel_id, user.id, payload)
        .await
        .map_err(|e| {
            eprintln!("Error sending message: {}", e);
            StatusCode::BAD_REQUEST
        })?;

    // Broadcast message to hub
    let ws_msg = WsMessage {
        message_type: "new_message".to_string(),
        payload: serde_json::to_value(&message).unwrap(),
    };
    hub.broadcast(channel_id, ws_msg);

    Ok((StatusCode::CREATED, Json(message)))
}

pub async fn list_channel_members_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(channel_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    let members = service::list_channel_members(&db, channel_id, user.id)
        .await
        .map_err(|e| {
            eprintln!("Error listing members: {}", e);
            StatusCode::BAD_REQUEST
        })?;
    Ok((StatusCode::OK, Json(members)))
}

pub async fn update_channel_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(channel_id): Path<Uuid>,
    Json(payload): Json<UpdateChannelRequest>,
) -> Result<(StatusCode, Json<crate::models::messaging::Channel>), StatusCode> {
    let channel = service::update_channel(&db, channel_id, user.id, payload)
        .await
        .map_err(|e| {
            eprintln!("Error updating channel: {}", e);
            StatusCode::BAD_REQUEST
        })?;
    Ok((StatusCode::OK, Json(channel)))
}

pub async fn remove_member_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((channel_id, target_user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    service::remove_member(&db, channel_id, user.id, target_user_id)
        .await
        .map_err(|e| {
            eprintln!("Error removing member: {}", e);
            StatusCode::BAD_REQUEST
        })?;
    Ok(StatusCode::OK)
}
