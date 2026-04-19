use chrono::Utc;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::Event;
#[derive(Row, Serialize)]
pub struct ClickRow {
    #[serde(with = "clickhouse::serde::uuid")]
    pub event_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub user_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub product_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub session_id: Uuid,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub timestamp: chrono::DateTime<Utc>,
    pub source: String,
    pub position: u32,
    pub category: String,
}

#[derive(Row, Serialize)]
pub struct ViewRow {
    #[serde(with = "clickhouse::serde::uuid")]
    pub event_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub user_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub product_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub session_id: Uuid,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub timestamp: chrono::DateTime<Utc>,
    pub source: String,
    pub position: u32,
    pub category: String,
}

#[derive(Row, Serialize)]
pub struct PurchaseRow {
    #[serde(with = "clickhouse::serde::uuid")]
    pub event_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub user_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub product_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub session_id: Uuid,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub timestamp: chrono::DateTime<Utc>,
    pub source: String,
    pub position: u32,
    pub category: String,
    pub amount: u32,
}

#[derive(Row, Deserialize, Serialize)]
pub struct TopProductRow {
    #[serde(with = "clickhouse::serde::uuid")]
    pub product_id: Uuid,
    pub count: u64,
}

impl From<Event> for PurchaseRow {
    fn from(value: Event) -> Self {
        PurchaseRow {
            event_id: Uuid::new_v4(),
            user_id: value.user_id,
            product_id: value.product_id,
            session_id: value.session_id,
            timestamp: value.timestamp,
            source: value.metadata.source.unwrap_or_default(),
            position: value.metadata.position.unwrap_or_default(),
            category: value.metadata.category.unwrap_or_default(),
            amount: value.metadata.amount.unwrap_or_default(),
        }
    }
}

impl From<Event> for ClickRow {
    fn from(value: Event) -> Self {
        ClickRow {
            event_id: Uuid::new_v4(),
            user_id: value.user_id,
            product_id: value.product_id,
            session_id: value.session_id,
            timestamp: value.timestamp,
            source: value.metadata.source.unwrap_or_default(),
            position: value.metadata.position.unwrap_or_default(),
            category: value.metadata.category.unwrap_or_default(),
        }
    }
}

impl From<Event> for ViewRow {
    fn from(value: Event) -> Self {
        ViewRow {
            event_id: Uuid::new_v4(),
            user_id: value.user_id,
            product_id: value.product_id,
            session_id: value.session_id,
            timestamp: value.timestamp,
            source: value.metadata.source.unwrap_or_default(),
            position: value.metadata.position.unwrap_or_default(),
            category: value.metadata.category.unwrap_or_default(),
        }
    }
}
