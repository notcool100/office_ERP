use crate::ws::hub::{Hub, WsMessage};
use axum::extract::ws as ax_ws;
use axum::{
    Extension,
    extract::{Path, State, WebSocketUpgrade, ws::WebSocket},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use uuid::Uuid;

pub async fn meeting_ws_handler(
    ws: WebSocketUpgrade,
    State(hub): State<Arc<Hub>>,
    Extension(user): Extension<crate::models::user::User>,
    Path(meeting_id): Path<Uuid>,
) -> Response {
    ws.on_upgrade(move |socket| handle_meeting_socket(socket, hub, meeting_id, user.id))
}

async fn handle_meeting_socket(socket: WebSocket, hub: Arc<Hub>, meeting_id: Uuid, user_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = hub.subscribe(meeting_id);

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

    let recv_hub = hub.clone();
    let mut recv_task = tokio::spawn(async move {
        // Announce this peer to the room before entering the read loop, so
        // participants already connected know to initiate a WebRTC offer.
        recv_hub.broadcast(
            meeting_id,
            WsMessage {
                message_type: "peer_online".to_string(),
                payload: serde_json::json!({ "user_id": user_id }),
            },
        );

        while let Some(Ok(msg)) = receiver.next().await {
            if let ax_ws::Message::Text(text) = msg {
                if let Ok(serde_json::Value::Object(mut obj)) =
                    serde_json::from_str::<serde_json::Value>(&text)
                {
                    // Never trust a client-supplied sender id: overwrite with
                    // the server-known authenticated user id.
                    obj.insert(
                        "from_user_id".to_string(),
                        serde_json::Value::String(user_id.to_string()),
                    );

                    recv_hub.broadcast(
                        meeting_id,
                        WsMessage {
                            message_type: "signal".to_string(),
                            payload: serde_json::Value::Object(obj),
                        },
                    );
                }
            }
        }

        recv_hub.broadcast(
            meeting_id,
            WsMessage {
                message_type: "peer_offline".to_string(),
                payload: serde_json::json!({ "user_id": user_id }),
            },
        );
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
