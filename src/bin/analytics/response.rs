use event_analytics::clickhouse_rows::TopProductRow;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UserActivityResponse {
    pub user_id: Uuid,
    pub from: String,
    pub to: String,
    pub events: EventCounts,
    pub top_products: Vec<ProductActivity>,
}

#[derive(Debug, Serialize, Default)]
pub struct EventCounts {
    pub clicks: u64,
    pub views: u64,
    pub purchases: u64,
}

#[derive(Debug, Serialize)]
pub struct ProductActivity {
    pub product_id: Uuid,
    pub event_counts: EventCounts,
}

#[derive(Debug, Serialize)]
pub struct ProductsResponse {
    pub period: String,
    pub metric: String,
    pub(crate) items: Vec<ProductItemResponse>,
}

#[derive(Debug, Serialize)]
pub struct ProductItemResponse {
    pub product_id: Uuid,
    pub count: u64,
}

impl From<&TopProductRow> for ProductItemResponse {
    fn from(value: &TopProductRow) -> Self {
        Self {
            product_id: value.product_id,
            count: value.count,
        }
    }
}
