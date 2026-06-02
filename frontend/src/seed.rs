use common::types::GameMode;

use crate::mode::mode_str;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub(crate) fn seed_from_values(values: [f64; 6]) -> String {
    values.iter()
        .map(|&v| CHARSET[(v * CHARSET.len() as f64) as usize] as char)
        .collect()
}

pub(crate) fn build_share_url(origin: &str, seed: &str, mode: &GameMode) -> String {
    format!("{}/?seed={}&mode={}", origin, seed, mode_str(mode))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn generate_seed() -> String {
    seed_from_values([
        js_sys::Math::random(),
        js_sys::Math::random(),
        js_sys::Math::random(),
        js_sys::Math::random(),
        js_sys::Math::random(),
        js_sys::Math::random(),
    ])
}

// Stub for native (test) builds — generate_seed is only called from browser event handlers.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn generate_seed() -> String {
    unreachable!("generate_seed is only called in the browser")
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- seed_from_values ---

    #[test]
    fn seed_from_values_produces_six_chars() {
        let s = seed_from_values([0.0, 0.1, 0.2, 0.5, 0.8, 0.99]);
        assert_eq!(s.len(), 6);
    }

    #[test]
    fn seed_from_values_all_chars_are_uppercase_alphanumeric() {
        let s = seed_from_values([0.0, 0.1, 0.3, 0.5, 0.7, 0.99]);
        for ch in s.chars() {
            assert!(ch.is_ascii_uppercase() || ch.is_ascii_digit(), "unexpected char: {ch}");
        }
    }

    #[test]
    fn seed_from_values_same_input_same_output() {
        let input = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6];
        assert_eq!(seed_from_values(input), seed_from_values(input));
    }

    #[test]
    fn seed_from_values_different_inputs_different_outputs() {
        let a = seed_from_values([0.0, 0.1, 0.2, 0.3, 0.4, 0.5]);
        let b = seed_from_values([0.9, 0.8, 0.7, 0.6, 0.5, 0.4]);
        assert_ne!(a, b);
    }

    #[test]
    fn seed_from_values_near_zero_maps_to_first_charset_char() {
        let s = seed_from_values([0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&s, "AAAAAA");
    }

    #[test]
    fn seed_from_values_known_input_known_output() {
        // 0.999 * 36 = 35.964 → index 35 → '9'
        let s = seed_from_values([0.999, 0.999, 0.999, 0.999, 0.999, 0.999]);
        assert_eq!(&s, "999999");
    }

    // --- build_share_url ---

    #[test]
    fn build_share_url_contains_seed() {
        let url = build_share_url("https://linguaguessr.io", "ABC123", &GameMode::Medium);
        assert!(url.contains("ABC123"));
    }

    #[test]
    fn build_share_url_contains_mode() {
        let url = build_share_url("https://linguaguessr.io", "ABC123", &GameMode::Hard);
        assert!(url.contains("mode=hard"));
    }

    #[test]
    fn build_share_url_format() {
        let url = build_share_url("https://linguaguessr.io", "XY9012", &GameMode::Easy);
        assert_eq!(url, "https://linguaguessr.io/?seed=XY9012&mode=easy");
    }

    #[test]
    fn build_share_url_medium_mode() {
        let url = build_share_url("https://linguaguessr.io", "HELLO1", &GameMode::Medium);
        assert_eq!(url, "https://linguaguessr.io/?seed=HELLO1&mode=medium");
    }
}
