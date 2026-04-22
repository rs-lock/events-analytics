CREATE DATABASE IF NOT EXISTS events;

 CREATE TABLE IF NOT EXISTS events.clicks (
    event_id UUID,
    user_id UUID,
    product_id UUID,
    session_id UUID,
    timestamp DateTime64(3),
    source String,
    position UInt32,
    category String,
    date Date DEFAULT toDate(timestamp)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (user_id, timestamp);

 CREATE TABLE IF NOT EXISTS events.views (
    event_id UUID,
    user_id UUID,
    product_id UUID,
    session_id UUID,
    timestamp DateTime64(3),
    source String,
    position UInt32,
    category String,
    date Date DEFAULT toDate(timestamp)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (user_id, timestamp);

 CREATE TABLE IF NOT EXISTS events.purchases (
    event_id UUID,
    user_id UUID,
    product_id UUID,
    session_id UUID,
    timestamp DateTime64(3),
    source String,
    position UInt32,
	amount UInt32,
    category String,
    date Date DEFAULT toDate(timestamp)
) ENGINE = MergeTree()
PARTITION BY toYYYYMM(date)
ORDER BY (user_id, timestamp);

CREATE MATERIALIZED VIEW events.mv_revenue_hourly
ENGINE = SummingMergeTree()
ORDER BY (product_id, hour)
AS SELECT
    product_id,
    toStartOfHour(timestamp) AS hour,
    sum(amount) AS revenue,
    count() AS purchase_count
FROM events.purchases
GROUP BY product_id, hour;

CREATE MATERIALIZED VIEW events.mv_views_hourly
ENGINE = SummingMergeTree()
ORDER BY (product_id, hour)
AS SELECT
    product_id,
    toStartOfHour(timestamp) AS hour,
    count() AS count
FROM events.views
GROUP BY product_id, hour;