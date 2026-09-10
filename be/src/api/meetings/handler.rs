use crate::{
    api::meetings::{
        dto::{CreateMeetingRequest, UpdateMinutesRequest},
        service,
    },
    db::Db,
    models::user::User,
    ws::hub::{Hub, WsMessage},
};
use axum::{
    Extension, Json,
    body::Body,
    extract::{Multipart, Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

/// Maps anyhow errors from the attachments/minutes service functions to the
/// appropriate status code. Everything else in this module collapses errors
/// to a single bare StatusCode, but these new endpoints need to distinguish
/// an authorization failure (participant check) from a not-found from a
/// generic bad request, so we match on the error message text rather than
/// introducing a typed error enum for just this code path.
fn map_meetings_error(e: &anyhow::Error) -> StatusCode {
    let msg = e.to_string();
    if msg.contains("Not a participant of this meeting") {
        StatusCode::FORBIDDEN
    } else if msg.contains("not found") {
        StatusCode::NOT_FOUND
    } else if msg.contains("Only the uploader or the meeting host") {
        StatusCode::FORBIDDEN
    } else {
        StatusCode::BAD_REQUEST
    }
}

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
            for member_id in &member_ids {
                if *member_id != user.id {
                    hub.send_to_user(*member_id, ws_msg.clone());
                    hub.send_to_user(*member_id, meeting_msg.clone());
                }
            }

            let title = meeting.title.clone();
            crate::api::notifications::service::notify_many(
                &db,
                Some(&hub),
                &member_ids,
                Some(user.id),
                || {
                    crate::api::notifications::dto::NewNotification::new(
                        crate::api::notifications::dto::kind::MEETING,
                        format!("Meeting started: {}", title),
                    )
                    .entity("meeting", meeting.id)
                },
            )
            .await;
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

pub async fn list_attachments_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(meeting_id): Path<Uuid>,
) -> Response {
    match service::list_attachments(&db, meeting_id, user.id).await {
        Ok(items) => Json(items).into_response(),
        Err(e) => {
            eprintln!("Error listing meeting attachments: {}", e);
            map_meetings_error(&e).into_response()
        }
    }
}

pub async fn upload_attachment_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(meeting_id): Path<Uuid>,
    mut multipart: Multipart,
) -> Response {
    let mut category = String::new();
    let mut file_name = String::new();
    let mut content_type_str = String::from("application/octet-stream");
    let mut file_data: Vec<u8> = vec![];

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "file" => {
                file_name = field.file_name().unwrap_or("upload").to_string();
                if let Some(ct) = field.content_type() {
                    content_type_str = ct.to_string();
                }
                match field.bytes().await {
                    Ok(b) => file_data = b.to_vec(),
                    Err(_) => {
                        return (StatusCode::BAD_REQUEST, "Failed to read file").into_response();
                    }
                }
            }
            "category" => {
                category = field.text().await.unwrap_or_default();
            }
            _ => {}
        }
    }

    if file_data.is_empty() {
        return (StatusCode::BAD_REQUEST, "No file provided").into_response();
    }

    match service::upload_attachment(
        &db,
        meeting_id,
        user.id,
        category,
        file_name,
        content_type_str,
        file_data,
    )
    .await
    {
        Ok(attachment) => (StatusCode::CREATED, Json(attachment)).into_response(),
        Err(e) => {
            eprintln!("Error uploading meeting attachment: {}", e);
            map_meetings_error(&e).into_response()
        }
    }
}

pub async fn delete_attachment_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((meeting_id, attachment_id)): Path<(Uuid, Uuid)>,
) -> Response {
    match service::delete_attachment(&db, meeting_id, attachment_id, user.id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Error deleting meeting attachment: {}", e);
            map_meetings_error(&e).into_response()
        }
    }
}

pub async fn download_attachment_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path((meeting_id, attachment_id)): Path<(Uuid, Uuid)>,
) -> Response {
    let attachment =
        match service::get_attachment_file(&db, meeting_id, attachment_id, user.id).await {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Error fetching meeting attachment: {}", e);
                return map_meetings_error(&e).into_response();
            }
        };

    let file = match File::open(&attachment.file_path).await {
        Ok(f) => f,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found on disk").into_response(),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let disposition = format!("inline; filename=\"{}\"", attachment.file_name);

    (
        [
            (header::CONTENT_TYPE, attachment.content_type),
            (header::CONTENT_DISPOSITION, disposition),
        ],
        body,
    )
        .into_response()
}

pub async fn get_minutes_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(meeting_id): Path<Uuid>,
) -> Response {
    match service::get_minutes(&db, meeting_id, user.id).await {
        Ok(minutes) => Json(minutes).into_response(),
        Err(e) => {
            eprintln!("Error fetching meeting minutes: {}", e);
            map_meetings_error(&e).into_response()
        }
    }
}

pub async fn update_minutes_handler(
    Extension(db): Extension<Db>,
    Extension(user): Extension<User>,
    Path(meeting_id): Path<Uuid>,
    Json(payload): Json<UpdateMinutesRequest>,
) -> Response {
    match service::update_minutes(&db, meeting_id, user.id, payload.content).await {
        Ok(minutes) => Json(minutes).into_response(),
        Err(e) => {
            eprintln!("Error updating meeting minutes: {}", e);
            map_meetings_error(&e).into_response()
        }
    }
}
