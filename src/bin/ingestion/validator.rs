use event_analytics::errors::EventError;
use event_analytics::models::EventType;

pub fn validate_event_type(event_type: &str) -> Result<EventType, EventError> {
    match event_type.to_lowercase().as_str() {
        "click" => Ok(EventType::Click),
        "purchase" => Ok(EventType::Purchase),
        "view" => Ok(EventType::View),
        _ => Err(EventError::UnknownEvent),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_event_type_valid() {
        assert!(validate_event_type("click").is_ok());
        assert!(validate_event_type("view").is_ok());
        assert!(validate_event_type("purchase").is_ok());
    }

    #[test]
    fn test_validate_event_type_invalid() {
        assert!(validate_event_type("").is_err());
        assert!(validate_event_type("invalid").is_err());
    }
}
