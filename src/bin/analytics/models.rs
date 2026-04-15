use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Interval {
    OneHour,
    TwentyFourHours,
    SevenDays,
}

impl Interval {
    pub fn as_param(&self) -> &'static str {
        match self {
            Self::OneHour => "1h",
            Self::TwentyFourHours => "24h",
            Self::SevenDays => "7d",
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::OneHour => write!(f, "1 HOUR"),
            Interval::TwentyFourHours => write!(f, "24 HOUR"),
            Interval::SevenDays => write!(f, "7 DAY"),
        }
    }
}

pub enum Metric {
    Clicks,
    Views,
    Purchases,
}

impl Metric {
    pub fn table(&self) -> &'static str {
        match self {
            Metric::Clicks => "clicks",
            Metric::Views => "views",
            Metric::Purchases => "purchases",
        }
    }
}
