use crate::models::{Interval, Metric};

pub fn validate_metric(metric: &str) -> Option<Metric> {
    match metric {
        "clicks" => Some(Metric::Clicks),
        "views" => Some(Metric::Views),
        "purchases" => Some(Metric::Purchases),
        _ => None,
    }
}

pub fn validate_period(metric: &str) -> Option<Interval> {
    match metric.to_lowercase().as_str() {
        "1h" => Some(Interval::OneHour),
        "24h" => Some(Interval::TwentyFourHours),
        "7d" => Some(Interval::SevenDays),
        _ => None,
    }
}
