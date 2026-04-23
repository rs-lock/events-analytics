pub mod clickhouse_rows;
pub mod errors;
pub mod models;

pub const TOPICS: &[&str] = &["events.clicks", "events.views", "events.purchases"];

pub fn env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{key} not set"))
}
