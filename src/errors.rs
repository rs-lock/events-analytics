use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventError {
    #[error("Invalid timestamp")]
    Timestamp,
    #[error("Unknown event")]
    UnknownEvent,
    #[error("kafka unavailable")]
    KafkaError,
    #[error("serialization error")]
    SerError,
}

impl ResponseError for EventError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Timestamp => StatusCode::BAD_REQUEST,
            Self::UnknownEvent => StatusCode::BAD_REQUEST,
            Self::KafkaError => StatusCode::SERVICE_UNAVAILABLE,
            Self::SerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(serde_json::json!({"error": self.to_string()}))
    }
}

#[derive(Debug, Error)]
pub enum WorkerError {
    #[error("topic not found")]
    TopicNotFound,
    #[error("clickhose error: {0}")]
    ClickHouse(#[from] clickhouse::error::Error),
}
