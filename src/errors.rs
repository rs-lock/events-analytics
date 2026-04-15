use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum EventError {
    Timestamp,
    UnknownEvent,
    KafkaError,
    SerError,
}

impl ResponseError for EventError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    /// Creates full response for error.
    ///
    /// By default, the generated response uses a 500 Internal Server Error status code, a
    /// `Content-Type` of `text/plain`, and the body is set to `Self`'s `Display` impl.
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            EventError::Timestamp => HttpResponse::new(StatusCode::BAD_REQUEST),
            EventError::UnknownEvent => {
                HttpResponse::build(StatusCode::BAD_REQUEST).json(serde_json::json!({"code": 400,
                            "success": false,
                            "payload": {
                }
                }))
            }
            Self::KafkaError => HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).json(
                serde_json::json!({"code": 503,
                            "success": false,
                            "payload": {
                                "error": "kafka not running"
                }
                }),
            ),
            Self::SerError => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl Display for EventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timestamp => write!(f, "INVALID TIMESTAMP"),
            Self::UnknownEvent => write!(f, "Unknown event"),
            Self::KafkaError => write!(f, "kafka unavailable"),
            Self::SerError => write!(f, "serialization error"),
        }
    }
}

impl std::error::Error for EventError {}

#[derive(Debug)]
pub enum WorkerError {
    TopicNotFound,
    ClickHouse(clickhouse::error::Error),
}

impl Display for WorkerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TopicNotFound => write!(f, "topic not found"),
            Self::ClickHouse(e) => write!(f, "clickhose error: {e}"),
        }
    }
}
impl std::error::Error for WorkerError {}

impl From<clickhouse::error::Error> for WorkerError {
    fn from(e: clickhouse::error::Error) -> Self {
        WorkerError::ClickHouse(e)
    }
}
