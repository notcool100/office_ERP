pub mod hub;

use crate::ws::hub::Hub;
use axum::extract::ws as ax_ws;
use axum::{
    Extension,
    extract::{Path, State, WebSocketUpgrade, ws::WebSocket},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use uuid::Uuid;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(hub): State<Arc<Hub>>,
    Path(channel_id): Path<Uuid>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, hub, channel_id))
}

pub async fn notification_ws_handler(
    ws: WebSocketUpgrade,
    State(hub): State<Arc<Hub>>,
    Extension(user): Extension<crate::models::user::User>,
) -> Response {
    ws.on_upgrade(move |socket| handle_notification_socket(socket, hub, user.id))
}

async fn handle_socket(socket: WebSocket, hub: Arc<Hub>, channel_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = hub.subscribe(channel_id);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if sender
                .send(ax_ws::Message::Text(json.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let mut recv_task =
        tokio::spawn(async move { while let Some(Ok(_)) = receiver.next().await {} });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn handle_notification_socket(socket: WebSocket, hub: Arc<Hub>, user_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = hub.subscribe_user(user_id);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if sender
                .send(ax_ws::Message::Text(json.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let mut recv_task =
        tokio::spawn(async move { while let Some(Ok(_)) = receiver.next().await {} });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
