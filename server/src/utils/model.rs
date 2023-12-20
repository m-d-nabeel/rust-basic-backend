use std::sync::{Arc, Mutex};

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub message_text: String,
    pub timestamp: String,
}

impl Message {
    pub(crate) fn clone(&self) -> Message {
        Message {
            id: self.id.clone(),
            message_text: self.message_text.clone(),
            timestamp: self.timestamp.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MessageForCreate {
    pub message_text: String,
}

#[derive(Clone)]
pub struct ModelController {
    pub message_store: Arc<Mutex<Vec<Option<Message>>>>,
}

impl ModelController {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            message_store: Arc::default()
        })
    }
}

impl ModelController {
    pub async fn create_message(&self, message_fc: MessageForCreate) -> anyhow::Result<Message> {
        let mut store = self.message_store.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let message = Message {
            id,
            message_text: message_fc.message_text,
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        store.push(Some(message.clone()));
        Ok(message)
    }
    pub async fn get_messages(&self) -> Result<Vec<Message>> {
        let store = self.message_store.lock().unwrap();
        let messages = store.iter().filter_map(|t| t.clone()).collect();
        Ok(messages)
    }
}