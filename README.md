
# Overview

Event-analytics is an event-driven project inspired by real problems of high-load web-apps, such as analytics dashboards, online shops where RPS(requests per second) can reach tens of thousands and the application should work within low latency.

---
#  Architecture 

The application design is based on Event-Driven Architecture which allows us to distribute all load between separated web applications.
At a high level the system could be described in several components that are shown in the diagram below. 
```
⏺ ┌─────────┐       ┌─────────────┐       ┌───────────┐       ┌─────────┐
  │ Clients │──────▶│ Ingestion   │──────▶│   Kafka   │──────▶│ Workers │
  │         │ HTTP  │ API         │ async │           │consume│ (pool)  │
  └─────────┘       └─────────────┘       └───────────┘       └────┬────┘
                                                                    │
                                                                    │ batch insert
                                                                    ▼
  ┌─────────────┐                                           ┌────────────┐
  │ Analytics   │◀──────────────────────────────────────────│ ClickHouse │
  │ API         │              SQL queries                  │            │
  └─────────────┘                                           └────────────┘
        │
        │ HTTP
        ▼
  ┌─────────┐
  │ Clients │
  └─────────┘
```
1) Ingestion API
2) Kafka broker
3) Workers binary
4) Clickhouse
5) Analytics API

As we can see, clients are sending requests with some payload to the Ingestion API and this API transmits this data through async message broker - kafka, then these messages are listened to and consumed by a workers application. The workers app includes a worker pool, so all messages will be distributed between workers, and then all the data is inserted in SQL table. As a database we chose Clickhouse, because we have to face many events at a time, and rate of inserting in the table is very high. The last instance of event analytics is Analytics API. This API allows clients to get all the data, such as realtime data, historical data of needed events and more.

---
# Quickstart

1) Prerequisites:  Docker, docker-compose
2) Clone and run: `docker-compose up -d`
3) Wait for services
4) Initialize DB with command: 
    `docker-compose exec -T clickhouse clickhouse-client --multiquery < init.sql`
5) Verify Ingestion API is running: `curl http://localhost:8080/health`
6) Send test event with current date via POST request:
 `curl -X POST http://localhost:8080/api/v1/events/view \       
    -H "Content-Type: application/json" \
    -d '{
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "product_id": "660e8400-e29b-41d4-a716-446655440001",
      "session_id": "770e8400-e29b-41d4-a716-446655440002",
      "timestamp": "2026-04-22T10:45:00Z",
      "metadata": {
        "source": "test",
        "position": 5,
        "category": "test"
      }
    }'`

7)  Verify that the analytics api works and events are stored in database:
    `curl "http://localhost:8081/api/v1/analytics/realtime-stats"`  

# API reference

### Ingestion API
1) POST /api/v1/events/click
2) POST /api/v1/events/view
3) POST /api/v1/events/purchase

  **Request body:**
  ```json
  {
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "product_id": "660e8400-e29b-41d4-a716-446655440001",
    "session_id": "770e8400-e29b-41d4-a716-446655440002",
    "timestamp": "2026-04-22T10:45:00Z",
    "metadata": {
      "source": "homepage",
      "position": 5,
      "category": "electronics"
    }
  }
  ```

### Analytics API
1) GET /api/v1/analytics/top-products

- Query params:

  | Param  | Required | Example     |
  |--------|----------|-------------|
  | metric | yes      | clicks      |
  | period | yes      | 1h, 24h, 7d |
  | limit  | no       | 10          |
  
2) GET /api/v1/analytics/user-activity/{user_id}

- Query params:

  | Param  | Required | Example              |
  |--------|----------|----------------------|
  | from   | yes      | 2026-04-22T00:00:00Z |
  | to     | yes      | 2026-04-23T00:00:00Z |

3) GET /api/v1/analytics/conversion-rate

- Query params:

  | Param  | Required | Example              |
  |--------|----------|----------------------|
  | from   | yes      | 2026-04-22T00:00:00Z |
  | to     | yes      | 2026-04-23T00:00:00Z |

4) GET /api/v1/analytics/realtime-stats

- No query params

# Configuration

Copy `.env.example` to `.env` and adjust as needed.

  ```env
  KAFKA_BROKERS=localhost:9092
  INGESTION_BIND=127.0.0.1:8080
  ANALYTICS_BIND=127.0.0.1:8081
  CLICKHOUSE_URL=http://localhost:8123
  CLICKHOUSE_USER=default
  CLICKHOUSE_PASSWORD=
  CLICKHOUSE_DATABASE=events
  BATCH_SIZE=1000
  FLUSH_INTERVAL_SECONDS=5
  RUST_LOG=info
  ```

  - KAFKA_BROKERS — Kafka bootstrap servers
  - INGESTION_BIND — Ingestion API bind address
  - ANALYTICS_BIND — Analytics API bind address
  - CLICKHOUSE_URL — ClickHouse HTTP endpoint
  - CLICKHOUSE_USER / CLICKHOUSE_PASSWORD — ClickHouse credentials
  - CLICKHOUSE_DATABASE — Database name
  - BATCH_SIZE — Worker batch size before flush
  - FLUSH_INTERVAL_SECONDS — Max seconds before flush
  - RUST_LOG — Log level (trace, debug, info, warn, error)

# Performance 

Performance was measured using wrk with 8 threads, 500 connections, for 60 seconds
`wrk -t 8 -c 500 -d 60s --latency -s loadtesting/post.lua \ http://localhost:8080/api/v1/events/view `

Test script located at ./loadtesting/post.lua

These are results: 

  | Metric | Value |
  |--------|-------|
  | Requests/sec | **40,888** |
  | Avg latency | 12.12ms |
  | p50 | 11.98ms |
  | p75 | 12.36ms |
  | p90 | 12.97ms |
  | p99 | 15.78ms |
  | Max latency | 42.39ms |
  | Total requests | 2,454,461 |

  **Target vs Actual:**

  | Metric | Target | Actual |
  |--------|--------|--------|
  | RPS | 3,000+ | 40,888 |
  | p99 latency | <150ms | 15.78ms |