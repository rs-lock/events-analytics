use std::collections::HashMap;

use clickhouse::Client;
use event_analytics::{
    TOPICS,
    clickhouse_rows::{ClickRow, PurchaseRow, ViewRow},
    errors::WorkerError,
    models::Event,
};

use crate::retry::run_with_backoff;

pub async fn flush_clicks_batch(batch: &Vec<Event>, client: &Client) -> Result<(), WorkerError> {
    let mut insert = client.insert::<ClickRow>("clicks").await?;

    for e in batch {
        insert.write(&ClickRow::from(e.clone())).await?;
    }
    insert.end().await.map_err(Into::into)
}

pub async fn flush_views_batch(batch: &Vec<Event>, client: &Client) -> Result<(), WorkerError> {
    let mut insert = client.insert::<ViewRow>("views").await?;

    for e in batch {
        insert.write(&ViewRow::from(e.clone())).await?;
    }
    insert.end().await.map_err(Into::into)
}

pub async fn flush_purchases_batch(batch: &Vec<Event>, client: &Client) -> Result<(), WorkerError> {
    let mut insert = client.insert::<PurchaseRow>("purchases").await?;
    for e in batch {
        insert.write(&PurchaseRow::from(e.clone())).await?;
    }
    insert.end().await.map_err(Into::into)
}

pub async fn flush_all(
    batch_map: &mut HashMap<&str, Vec<Event>>,
    client: &Client,
) -> Result<(), WorkerError> {
    for t in TOPICS.iter().copied() {
        let batch = batch_map.get_mut(t).unwrap();

        if batch.is_empty() {
            continue;
        }
        match t {
            "events.clicks" => run_with_backoff(|| flush_clicks_batch(batch, client)).await?,
            "events.views" => run_with_backoff(|| flush_views_batch(batch, client)).await?,
            "events.purchases" => run_with_backoff(|| flush_purchases_batch(batch, client)).await?,
            _ => {
                unreachable!()
            }
        }
        batch.clear();
    }
    Ok(())
}
