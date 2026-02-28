pub mod hub;

use axum::extract::ws as ax_ws;
use axum::{
    extract::{WebSocketUpgrade, ws::WebSocket, State},
    response::Response,
};
use std::sync::Arc;
use futures::{sink::SinkExt, stream::StreamExt};
use crate::ws::hub::Hub;
use uuid::Uuid;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(hub): State<Arc<Hub>>,
    axum::extract::Path(channel_id): axum::extract::Path<Uuid>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, hub, channel_id))
}

async fn handle_socket(socket: WebSocket, hub: Arc<Hub>, channel_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = hub.subscribe(channel_id);

    // Task to send messages from hub to websocket
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if sender.send(ax_ws::Message::Text(json.into())).await.is_err() {
                break;
            }
        }
    });

    // Task to receive messages from websocket (just to keep connection alive/handle close)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = receiver.next().await {
            // We could handle incoming socket messages here if needed
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
