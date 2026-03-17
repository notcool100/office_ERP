use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub message_type: String, // "new_message", "user_joined", etc.
    pub payload: serde_json::Value,
}

pub struct Hub {
    // Maps channel_id to a broadcast sender (for group chats)
    pub channels: Mutex<HashMap<Uuid, broadcast::Sender<WsMessage>>>,
    // Maps user_id to a broadcast sender (for personal notifications)
    pub users: Mutex<HashMap<Uuid, broadcast::Sender<WsMessage>>>,
}

impl Hub {
    pub fn new() -> Arc<Self> {
        Arc::new(Hub {
            channels: Mutex::new(HashMap::new()),
            users: Mutex::new(HashMap::new()),
        })
    }

    pub fn subscribe(&self, channel_id: Uuid) -> broadcast::Receiver<WsMessage> {
        let mut channels = self.channels.lock().unwrap();
        let sender = channels.entry(channel_id).or_insert_with(|| {
            let (tx, _rx) = broadcast::channel(100);
            tx
        });
        sender.subscribe()
    }

    pub fn subscribe_user(&self, user_id: Uuid) -> broadcast::Receiver<WsMessage> {
        let mut users = self.users.lock().unwrap();
        let sender = users.entry(user_id).or_insert_with(|| {
            let (tx, _rx) = broadcast::channel(100);
            tx
        });
        sender.subscribe()
    }

    pub fn broadcast(&self, channel_id: Uuid, msg: WsMessage) {
        let channels = self.channels.lock().unwrap();
        if let Some(sender) = channels.get(&channel_id) {
            let _ = sender.send(msg);
        }
    }

    pub fn send_to_user(&self, user_id: Uuid, msg: WsMessage) {
        let users = self.users.lock().unwrap();
        if let Some(sender) = users.get(&user_id) {
            let _ = sender.send(msg);
        }
    }
}
