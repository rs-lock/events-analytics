use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalyticsError {
    #[error("Missing period")]
    MissingPeriod,
    #[error("Missing metric")]
    MissingMetric,
    #[error("Invalid metric")]
    InvalidMetric,
    #[error("Invalid period")]
    InvalidPeriod,
    #[error("Invalid userid")]
    InvalidUserID,
    #[error("Invalid `from`")]
    InvalidFrom,
    #[error("Invalid `to`")]
    InvalidTo,
    #[error("Missing `from`")]
    MissingFrom,
    #[error("Missing `to`")]
    MissingTo,
    #[error("Clickhouse error {0}")]
    ClickHouse(#[from] clickhouse::error::Error),
}

impl ResponseError for AnalyticsError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::MissingPeriod
            | Self::MissingMetric
            | Self::InvalidMetric
            | Self::InvalidPeriod
            | Self::InvalidUserID
            | Self::MissingTo
            | Self::MissingFrom
            | Self::InvalidFrom
            | Self::InvalidTo => StatusCode::BAD_REQUEST,
            Self::ClickHouse(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        if let Self::ClickHouse(e) = self {
            tracing::error!(error = ?e, "clickhouse query failed");
        }
        HttpResponse::build(self.status_code()).json(serde_json::json!({"error": self.to_string()}))
    }
}
