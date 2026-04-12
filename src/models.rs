use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub session_id: Uuid,
    pub timestamp: chrono::DateTime<Utc>,
    pub metadata: serde_json::Value,
}

pub enum EventType {
    Click,
    View,
    Purchase,
}

impl EventType {
    pub fn as_topic(&self) -> String {
        match self {
            EventType::Click => String::from("events.clicks"),
            EventType::View => String::from("events.views"),
            EventType::Purchase => String::from("events.purchases"),
        }
    }
}
