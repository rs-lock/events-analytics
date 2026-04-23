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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_metric_valid() {
        assert!(validate_metric("clicks").is_some());
        assert!(validate_metric("views").is_some());
        assert!(validate_metric("purchases").is_some());
    }

    #[test]
    fn test_validate_metric_invalid() {
        assert!(validate_metric("invalid").is_none());
        assert!(validate_metric("").is_none());
    }

    #[test]
    fn test_validate_period_valid() {
        assert!(validate_period("1h").is_some());
        assert!(validate_period("24h").is_some());
        assert!(validate_period("7d").is_some());
    }

    #[test]
    fn test_validate_period_invalid() {
        assert!(validate_period("2h").is_none());
        assert!(validate_period("").is_none());
    }
}
