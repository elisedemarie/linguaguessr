use common::types::{GameMode, Language};

pub fn mode_str(mode: &GameMode) -> &'static str {
    match mode {
        GameMode::Easy   => "easy",
        GameMode::Medium => "medium",
        GameMode::Hard   => "hard",
        GameMode::Daily  => "daily",
    }
}

pub fn suggestion_pool(mode: &GameMode) -> &'static [Language] {
    match mode {
        GameMode::Easy              => Language::easy_pool(),
        GameMode::Medium            => Language::medium_pool(),
        GameMode::Hard | GameMode::Daily => Language::all(),
    }
}

pub fn show_score_breakdown(mode: &GameMode) -> bool {
    *mode != GameMode::Easy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_str_easy() { assert_eq!(mode_str(&GameMode::Easy), "easy"); }

    #[test]
    fn mode_str_medium() { assert_eq!(mode_str(&GameMode::Medium), "medium"); }

    #[test]
    fn mode_str_hard() { assert_eq!(mode_str(&GameMode::Hard), "hard"); }

    #[test]
    fn show_score_breakdown_is_false_for_easy() {
        assert!(!show_score_breakdown(&GameMode::Easy));
    }

    #[test]
    fn show_score_breakdown_is_true_for_medium() {
        assert!(show_score_breakdown(&GameMode::Medium));
    }

    #[test]
    fn show_score_breakdown_is_true_for_hard() {
        assert!(show_score_breakdown(&GameMode::Hard));
    }

    #[test]
    fn mode_str_daily() { assert_eq!(mode_str(&GameMode::Daily), "daily"); }

    #[test]
    fn show_score_breakdown_is_true_for_daily() {
        assert!(show_score_breakdown(&GameMode::Daily));
    }

    #[test]
    fn suggestion_pool_daily_returns_full_75() {
        assert_eq!(suggestion_pool(&GameMode::Daily).len(), 75);
    }

    #[test]
    fn suggestion_pool_easy_returns_10() {
        assert_eq!(suggestion_pool(&GameMode::Easy).len(), 10);
    }

    #[test]
    fn suggestion_pool_medium_returns_30() {
        assert_eq!(suggestion_pool(&GameMode::Medium).len(), 30);
    }

    #[test]
    fn suggestion_pool_hard_returns_75() {
        assert_eq!(suggestion_pool(&GameMode::Hard).len(), 75);
    }
}
