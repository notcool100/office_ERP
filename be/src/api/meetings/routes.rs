use crate::api::meetings::handler::{
    create_meeting_handler, end_meeting_handler, get_meeting_handler, join_meeting_handler,
    leave_meeting_handler, list_my_meetings_handler, list_participants_handler,
};
use crate::ws::hub::Hub;
use axum::{Router, routing::get, routing::post};
use std::sync::Arc;

pub fn meetings_routes(hub: Arc<Hub>) -> Router {
    Router::new()
        .route("/", post(create_meeting_handler).get(list_my_meetings_handler))
        .route("/{meeting_id}", get(get_meeting_handler))
        .route("/{meeting_id}/join", post(join_meeting_handler))
        .route("/{meeting_id}/leave", post(leave_meeting_handler))
        .route("/{meeting_id}/end", post(end_meeting_handler))
        .route("/{meeting_id}/participants", get(list_participants_handler))
        .with_state(hub)
}
