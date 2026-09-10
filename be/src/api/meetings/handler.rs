use crate::{
    api::meetings::{dto::CreateMeetingRequest, service},
    db::Db,
    models::user::User,
    ws::hub::{Hub, WsMessage},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_meeting_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<CreateMeetingRequest>,
) -> Result<(StatusCode, Json<crate::api::meetings::dto::MeetingResponse>), StatusCode> {
    let channel_id = payload.channel_id;

    let (meeting, message) = service::create_meeting(&db, payload, user.id)
        .await
        .map_err(|e| {
            eprintln!("Error creating meeting: {}", e);
            StatusCode::BAD_REQUEST
        })?;

    if let (Some(channel_id), Some(message)) = (channel_id, &message) {
        let ws_msg = WsMessage {
            message_type: "new_message".to_string(),
            payload: serde_json::to_value(message).unwrap(),
        };
        hub.broadcast(channel_id, ws_msg.clone());

        if let Ok(member_ids) = crate::api::messaging::service::get_channel_member_ids(
            &db,
            channel_id,
        )
        .await
        {
            let meeting_msg = WsMessage {
                message_type: "meeting_created".to_string(),
                payload: serde_json::to_value(&meeting).unwrap(),
            };
            for member_id in member_ids {
                if member_id != user.id {
                    hub.send_to_user(member_id, ws_msg.clone());
                    hub.send_to_user(member_id, meeting_msg.clone());
                }
            }
        }
    }

    Ok((StatusCode::CREATED, Json(meeting)))
}

pub async fn get_meeting_handler(
    Extension(db): Extension<Db>,
    Path(meeting_id): Path<Uuid>,
) -> Result<(StatusCode, Json<crate::api::meetings::dto::MeetingResponse>), StatusCode> {
    let meeting = service::get_meeting(&db, meeting_id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(meeting)))
}

pub async fn join_meeting_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    State(hub): State<Arc<Hub>>,
    Path(meeting_id): Path<Uuid>,
) -> Result<
    (
        StatusCode,
        Json<Vec<crate::api::meetings::dto::ParticipantResponse>>,
    ),
    StatusCode,
> {
    let participants = service::join_meeting(&db, meeting_id, user.id)
        .await
        .map_err(|e| {
            eprintln!("Error joining meeting: {}", e);
            StatusCode::BAD_REQUEST
        })?;

    if let Some(joined) = participants.iter().find(|p| p.user_id == user.id) {
        let ws_msg = WsMessage {
            message_type: "participant_joined".to_string(),
            payload: serde_json::to_value(joined).unwrap(),
        };
        hub.broadcast(meeting_id, ws_msg);
    }

    Ok((StatusCode::OK, Json(participants)))
}

pub async fn leave_meeting_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    State(hub): State<Arc<Hub>>,
    Path(meeting_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    service::leave_meeting(&db, meeting_id, user.id)
        .await
        .map_err(|e| {
            eprintln!("Error leaving meeting: {}", e);
            StatusCode::BAD_REQUEST
        })?;

    let ws_msg = WsMessage {
        message_type: "participant_left".to_string(),
        payload: serde_json::json!({ "user_id": user.id }),
    };
    hub.broadcast(meeting_id, ws_msg);

    Ok(StatusCode::OK)
}

pub async fn end_meeting_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    State(hub): State<Arc<Hub>>,
    Path(meeting_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    service::end_meeting(&db, meeting_id, user.id)
        .await
        .map_err(|e| {
            eprintln!("Error ending meeting: {}", e);
            StatusCode::BAD_REQUEST
        })?;

    let ws_msg = WsMessage {
        message_type: "meeting_ended".to_string(),
        payload: serde_json::json!({}),
    };
    hub.broadcast(meeting_id, ws_msg);

    Ok(StatusCode::OK)
}

pub async fn list_participants_handler(
    Extension(db): Extension<Db>,
    Path(meeting_id): Path<Uuid>,
) -> Result<
    (
        StatusCode,
        Json<Vec<crate::api::meetings::dto::ParticipantResponse>>,
    ),
    StatusCode,
> {
    let participants = service::list_active_participants(&db, meeting_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(participants)))
}

pub async fn list_my_meetings_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
) -> Result<(StatusCode, Json<Vec<crate::api::meetings::dto::MeetingResponse>>), StatusCode> {
    let meetings = service::list_my_meetings(&db, user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::OK, Json(meetings)))
}
