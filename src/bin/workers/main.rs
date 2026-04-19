use std::{collections::HashMap, time::Duration};

use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
    error::KafkaError,
    message::BorrowedMessage,
};

use event_analytics::env;

use clickhouse::Client;

use event_analytics::{
    clickhouse_rows::{ClickRow, PurchaseRow, ViewRow},
    errors::WorkerError,
    models::Event,
};

const TOPICS: &[&str] = &["events.clicks", "events.views", "events.purchases"];

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let batch_size: usize = env("BATCH_SIZE")
        .parse()
        .expect("BATCH_SIZE must be a number");
    let flush_secs: u64 = env("FLUSH_INTERVAL_SECONDS")
        .parse()
        .expect("FLUSH_INTERVAL_SECONDS must be a number");

    let kafka = env("KAFKA_BROKERS");

    let ch_url = env("CLICKHOUSE_URL");
    let ch_user = env("CLICKHOUSE_USER");
    let ch_psw = env("CLICKHOUSE_PASSWORD");
    let ch_db = env("CLICKHOUSE_DATABASE");

    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", kafka);
    config.set("group.id", "workers");
    config.set("auto.offset.reset", "earliest");

    let consumer = config.create::<StreamConsumer>().unwrap();
    consumer
        .subscribe(TOPICS)
        .expect("Can't subscribe to specified topics");

    let mut interval = tokio::time::interval(Duration::from_secs(flush_secs));

    let mut batch_map: HashMap<&str, Vec<Event>> = TOPICS
        .iter()
        .map(|v| (*v, Vec::<Event>::with_capacity(batch_size)))
        .collect();

    let client = Client::default()
        .with_url(ch_url)
        .with_user(ch_user)
        .with_password(ch_psw)
        .with_database(ch_db);

    loop {
        tokio::select! {
                msg = consumer.recv() => {
                    if let Err(e) = handle_msg(msg, &mut batch_map, &client, batch_size).await {
                        eprintln!("handle error: {:?}", e);
                    }
                    }

            _ = interval.tick() => {
                eprintln!("interval tick - flushing");
                if let Err(e) = flush_all(&mut batch_map, &client).await {
                    eprintln!("flush error: {:?}", e);
                }
            }
        }
    }
}

fn extract_payload(data: &BorrowedMessage<'_>) -> Option<Event> {
    let payload = data.payload()?;
    serde_json::from_slice(payload).ok()
}

async fn handle_msg(
    msg: Result<BorrowedMessage<'_>, KafkaError>,
    batch_map: &mut HashMap<&str, Vec<Event>>,
    client: &Client,
    batch_size: usize,
) -> Result<(), WorkerError> {
    let v = match msg {
        Ok(v) => v,
        Err(e) => {
            eprintln!("kafka recv error: {:?}", e);
            return Ok(());
        }
    };

    let Some(event) = extract_payload(&v) else {
        eprintln!("failed to extract payload from topic {}", v.topic());
        return Ok(());
    };

    let topic = v.topic();
    let Some(vec) = batch_map.get_mut(topic) else {
        eprintln!("unknown topic: {}", topic);
        return Ok(());
    };

    eprintln!("queued event in {}, batch size: {}", topic, vec.len() + 1);
    vec.push(event);

    if vec.len() >= batch_size {
        match topic {
            "events.clicks" => flush_clicks_batch(vec, client).await?,
            "events.views" => flush_views_batch(vec, client).await?,
            "events.purchases" => flush_purchases_batch(vec, client).await?,
            _ => {}
        }
    }

    Ok(())
}

async fn flush_clicks_batch(batch: &mut Vec<Event>, client: &Client) -> Result<(), WorkerError> {
    let mut insert = client.insert::<ClickRow>("clicks").await?;

    for e in batch.drain(..) {
        insert.write(&e.into()).await?;
    }
    insert.end().await.map_err(Into::into)
}

async fn flush_views_batch(batch: &mut Vec<Event>, client: &Client) -> Result<(), WorkerError> {
    let mut insert = client.insert::<ViewRow>("views").await?;

    for e in batch.drain(..) {
        insert.write(&e.into()).await?;
    }
    insert.end().await.map_err(Into::into)
}

async fn flush_purchases_batch(batch: &mut Vec<Event>, client: &Client) -> Result<(), WorkerError> {
    let mut insert = client.insert::<PurchaseRow>("purchases").await?;

    for e in batch.drain(..) {
        insert.write(&e.into()).await?;
    }
    insert.end().await.map_err(Into::into)
}

async fn flush_all(
    batch_map: &mut HashMap<&str, Vec<Event>>,
    client: &Client,
) -> Result<(), WorkerError> {
    for t in TOPICS.iter().copied() {
        let batch = batch_map.get_mut(t).unwrap();
        match t {
            "events.clicks" => flush_clicks_batch(batch, client).await?,
            "events.views" => flush_views_batch(batch, client).await?,
            "events.purchases" => flush_purchases_batch(batch, client).await?,
            _ => {
                eprintln!("handle error: topic");
            }
        }
    }
    Ok(())
}
