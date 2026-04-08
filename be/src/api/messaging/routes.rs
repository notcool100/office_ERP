use crate::api::messaging::handler::{
    add_member_handler, create_channel_handler, get_channel_handler, list_channel_members_handler,
    list_channels_handler, list_messages_handler, remove_member_handler, send_message_handler,
    update_channel_handler,
};
use crate::ws::hub::Hub;
use axum::{Router, routing::delete, routing::get};
use std::sync::Arc;

pub fn messaging_routes(hub: Arc<Hub>) -> Router {
    Router::new()
        .route(
            "/channels",
            get(list_channels_handler).post(create_channel_handler),
        )
        .route(
            "/channels/{channel_id}",
            get(get_channel_handler).put(update_channel_handler),
        )
        .route(
            "/channels/{channel_id}/members",
            get(list_channel_members_handler).post(add_member_handler),
        )
        .route(
            "/channels/{channel_id}/members/{user_id}",
            delete(remove_member_handler),
        )
        .route(
            "/channels/{channel_id}/messages",
            get(list_messages_handler).post(send_message_handler),
        )
        .with_state(hub)
}
