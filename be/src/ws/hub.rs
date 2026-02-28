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
    // Maps channel_id to a broadcast sender
    pub channels: Mutex<HashMap<Uuid, broadcast::Sender<WsMessage>>>,
}

impl Hub {
    pub fn new() -> Arc<Self> {
        Arc::new(Hub {
            channels: Mutex::new(HashMap::new()),
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

    pub fn broadcast(&self, channel_id: Uuid, msg: WsMessage) {
        let channels = self.channels.lock().unwrap();
        if let Some(sender) = channels.get(&channel_id) {
            let _ = sender.send(msg);
        }
    }
}
