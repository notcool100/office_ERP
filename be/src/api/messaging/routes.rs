use axum::{Router, routing::get, routing::post, routing::put, routing::delete};
use crate::api::messaging::handler::{
    list_channels_handler, create_channel_handler, list_messages_handler, 
    send_message_handler, get_channel_handler, add_member_handler,
    list_channel_members_handler, update_channel_handler, remove_member_handler
};
use std::sync::Arc;
use crate::ws::hub::Hub;

pub fn messaging_routes(hub: Arc<Hub>) -> Router {
    Router::new()
        .route("/channels", get(list_channels_handler).post(create_channel_handler))
        .route("/channels/{channel_id}", get(get_channel_handler).put(update_channel_handler))
        .route("/channels/{channel_id}/members", get(list_channel_members_handler).post(add_member_handler))
        .route("/channels/{channel_id}/members/{user_id}", delete(remove_member_handler))
        .route("/channels/{channel_id}/messages", get(list_messages_handler).post(send_message_handler))
        .with_state(hub)
}
