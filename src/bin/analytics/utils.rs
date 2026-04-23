pub fn conversion_rate(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_rate_normal() {
        assert_eq!(conversion_rate(10, 100), 0.1);
    }

    #[test]
    fn test_conversion_rate_zero_denominator() {
        assert_eq!(conversion_rate(10, 0), 0.0);
    }
}
