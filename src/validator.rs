use crate::{errors::EventError, models::EventType};

pub fn validate_event_type(event_type: &str) -> Result<EventType, EventError> {
    match event_type.to_lowercase().as_str() {
        "click" => Ok(EventType::Click),
        "purchase" => Ok(EventType::Purchase),
        "view" => Ok(EventType::View),
        _ => Err(EventError::UnknownEvent),
    }
}
