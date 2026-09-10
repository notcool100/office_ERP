use crate::api::meetings::handler::{
    create_meeting_handler, delete_attachment_handler, download_attachment_handler,
    end_meeting_handler, get_meeting_handler, get_minutes_handler, join_meeting_handler,
    leave_meeting_handler, list_attachments_handler, list_my_meetings_handler,
    list_participants_handler, update_minutes_handler, upload_attachment_handler,
};
use crate::ws::hub::Hub;
use axum::{Router, routing::delete, routing::get, routing::post};
use std::sync::Arc;

pub fn meetings_routes(hub: Arc<Hub>) -> Router {
    Router::new()
        .route("/", post(create_meeting_handler).get(list_my_meetings_handler))
        .route("/{meeting_id}", get(get_meeting_handler))
        .route("/{meeting_id}/join", post(join_meeting_handler))
        .route("/{meeting_id}/leave", post(leave_meeting_handler))
        .route("/{meeting_id}/end", post(end_meeting_handler))
        .route("/{meeting_id}/participants", get(list_participants_handler))
        .route(
            "/{meeting_id}/attachments",
            get(list_attachments_handler).post(upload_attachment_handler),
        )
        .route(
            "/{meeting_id}/attachments/{attachment_id}",
            delete(delete_attachment_handler),
        )
        .route(
            "/{meeting_id}/attachments/{attachment_id}/download",
            get(download_attachment_handler),
        )
        .route(
            "/{meeting_id}/minutes",
            get(get_minutes_handler).put(update_minutes_handler),
        )
        .with_state(hub)
}
