use clickhouse::Client;
use event_analytics::clickhouse_rows::{CountRow, TopProductRow};
use uuid::Uuid;

use crate::models::{ClickHouseTimestamp, Interval, Metric};

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

pub async fn select_user_event_count(
    client: &Client,
    table: &str,
    user_id: Uuid,
    from: impl Into<ClickHouseTimestamp>,
    to: impl Into<ClickHouseTimestamp>,
) -> std::result::Result<u64, clickhouse::error::Error> {
    let sql = format!(
        "SELECT count() AS count 
        FROM {} 
        WHERE user_id = ? AND timestamp BETWEEN ? AND ?",
        table
    );

    let row = client
        .query(&sql)
        .bind(user_id)
        .bind(from.into())
        .bind(to.into())
        .fetch_one::<CountRow>()
        .await?;

    Ok(row.count)
}

pub async fn select_top_products_by_period(
    client: &Client,
    table: &str,
    user_id: Uuid,
    from: impl Into<ClickHouseTimestamp>,
    to: impl Into<ClickHouseTimestamp>,
) -> std::result::Result<Vec<TopProductRow>, clickhouse::error::Error> {
    let sql = format!(
        "SELECT product_id, count() AS count 
        FROM {} 
        WHERE user_id = ? AND timestamp BETWEEN ? AND ?
        GROUP BY product_id
        ORDER BY count DESC
        ",
        table
    );

    let rows: Vec<TopProductRow> = client
        .query(&sql)
        .bind(user_id)
        .bind(from.into())
        .bind(to.into())
        .fetch_all()
        .await?;

    Ok(rows)
}

pub async fn select_global_event_count(
    client: &Client,
    table: &str,
    from: impl Into<ClickHouseTimestamp>,
    to: impl Into<ClickHouseTimestamp>,
) -> std::result::Result<u64, clickhouse::error::Error> {
    let sql = format!(
        "SELECT count() AS count 
        FROM {}
        WHERE timestamp BETWEEN ? AND ?
    ",
        table
    );

    let row = client
        .query(&sql)
        .bind(from.into())
        .bind(to.into())
        .fetch_one::<CountRow>()
        .await?;

    Ok(row.count)
}
