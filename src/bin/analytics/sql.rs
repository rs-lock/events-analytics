use clickhouse::Client;
use event_analytics::clickhouse_rows::TopProductRow;

use crate::models::{Interval, Metric};

pub async fn select_top_products(
    client: &Client,
    lim: u32,
    metric: &Metric,
    interval: Interval,
) -> std::result::Result<Vec<TopProductRow>, clickhouse::error::Error> {
    let sql = format!(
        "SELECT product_id, count() AS count
         FROM {}
         WHERE timestamp >= now() - INTERVAL {}
         GROUP BY product_id
         ORDER BY count DESC
         LIMIT ?",
        metric.table(),
        interval
    );

    let rows: Vec<TopProductRow> = client.query(&sql).bind(lim).fetch_all().await?;

    Ok(rows)
}
