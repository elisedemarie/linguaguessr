pub fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_zero_returns_zero() {
        assert_eq!(ease_out_cubic(0.0), 0.0);
    }

    #[test]
    fn at_one_returns_one() {
        assert_eq!(ease_out_cubic(1.0), 1.0);
    }

    #[test]
    fn at_half_returns_0_875() {
        assert_eq!(ease_out_cubic(0.5), 0.875);
    }

    #[test]
    fn at_quarter_returns_correct_cubic_value() {
        assert_eq!(ease_out_cubic(0.25), 0.578_125);
    }

    #[test]
    fn is_monotonically_increasing() {
        assert!(ease_out_cubic(0.25) < ease_out_cubic(0.5));
        assert!(ease_out_cubic(0.5)  < ease_out_cubic(0.75));
    }
}
