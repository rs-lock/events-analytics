use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub event_type: String,
    pub user_id: String,
    pub timestamp: String,
    pub payload: serde_json::Value,
}

pub enum EventType {
    Click,
    View,
    Purchase,
}
