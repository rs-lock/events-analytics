use std::fmt::Display;

use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};

#[derive(Debug)]
pub enum AnalyticsError {
    MissingPeriod,
    MissingMetric,
    InvalidMetric,
    InvalidPeriod,
    InvalidUserID,
    InvalidFrom,
    InvalidTo,
    MissingFrom,
    MissingTo,
    ClickHouse(clickhouse::error::Error),
}

impl std::error::Error for AnalyticsError {}

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

impl Display for AnalyticsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyticsError::MissingPeriod => write!(f, "Missing period"),
            AnalyticsError::MissingMetric => write!(f, "Missing metric"),
            AnalyticsError::InvalidMetric => write!(f, "Invalid metric"),
            AnalyticsError::InvalidPeriod => write!(f, "Invalid period"),
            AnalyticsError::InvalidUserID => write!(f, "Invalid userid"),
            AnalyticsError::ClickHouse(e) => write!(f, "clickhouse error: {e}"),
            AnalyticsError::InvalidFrom => write!(f, "Invalid 'from' date"),
            AnalyticsError::InvalidTo => write!(f, "Invalid 'to' date"),
            AnalyticsError::MissingFrom => write!(f, "Missing 'from' date"),
            AnalyticsError::MissingTo => write!(f, "Missing 'to' date"),
        }
    }
}

impl From<clickhouse::error::Error> for AnalyticsError {
    fn from(e: clickhouse::error::Error) -> Self {
        AnalyticsError::ClickHouse(e)
    }
}
