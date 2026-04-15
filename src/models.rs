use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub session_id: Uuid,
    pub timestamp: chrono::DateTime<Utc>,
    pub metadata: Metadata,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metadata {
    pub source: Option<String>,
    pub position: Option<u32>,
    pub category: Option<String>,
    pub amount: Option<u64>,
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
