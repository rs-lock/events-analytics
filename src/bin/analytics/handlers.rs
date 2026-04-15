use actix_web::{
    HttpResponse,
    http::StatusCode,
    web::{self, Data},
};
use clickhouse::Client;
use event_analytics::clickhouse_rows::TopProductRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::AnalyticsError,
    sql::select_top_products,
    validation::{validate_metric, validate_period},
};

#[derive(Debug, Deserialize)]
pub struct TopProductsQuery {
    pub period: Option<String>,
    pub limit: Option<u16>,
    pub metric: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductsResponse {
    period: String,
    metric: String,
    items: Vec<ProductItemResponse>,
}

#[derive(Debug, Serialize)]
struct ProductItemResponse {
    product_id: Uuid,
    count: u64,
}

impl From<&TopProductRow> for ProductItemResponse {
    fn from(value: &TopProductRow) -> Self {
        Self {
            product_id: value.product_id,
            count: value.count,
        }
    }
}

pub async fn handle_top_products(
    query: web::Query<TopProductsQuery>,
    client: Data<Client>,
) -> Result<HttpResponse, AnalyticsError> {
    let query = query.into_inner();
    let metric = validate_metric(&query.metric.ok_or(AnalyticsError::MissingMetric)?)
        .ok_or(AnalyticsError::InvalidMetric)?;

    let period = validate_period(&query.period.ok_or(AnalyticsError::MissingPeriod)?)
        .ok_or(AnalyticsError::InvalidPeriod)?;

    let lim = query.limit.unwrap_or(10) as u32;

    let products = select_top_products(client.get_ref(), lim, &metric, period).await?;

    Ok(HttpResponse::Ok().json(ProductsResponse {
        period: period.as_param().to_string(),
        metric: metric.table().to_string(),
        items: products.iter().map(|p| p.into()).collect(),
    }))
}
