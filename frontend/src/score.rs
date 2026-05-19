use common::types::GameMode;

pub fn display_score(score: u32, mode: &GameMode) -> u32 {
    match mode {
        GameMode::Easy => score / 1000,
        _              => score,
    }
}

pub fn max_score(mode: &GameMode) -> u32 {
    match mode {
        GameMode::Easy => 5,
        _              => 5000,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_score_easy_divides_by_1000() {
        assert_eq!(display_score(3000, &GameMode::Easy), 3);
    }

    #[test]
    fn display_score_easy_correct_all_is_5() {
        assert_eq!(display_score(5000, &GameMode::Easy), 5);
    }

    #[test]
    fn display_score_easy_zero_is_0() {
        assert_eq!(display_score(0, &GameMode::Easy), 0);
    }

    #[test]
    fn display_score_medium_returns_raw() {
        assert_eq!(display_score(3750, &GameMode::Medium), 3750);
    }

    #[test]
    fn display_score_hard_returns_raw() {
        assert_eq!(display_score(2000, &GameMode::Hard), 2000);
    }

    #[test]
    fn max_score_easy_is_5() {
        assert_eq!(max_score(&GameMode::Easy), 5);
    }

    #[test]
    fn max_score_medium_is_5000() {
        assert_eq!(max_score(&GameMode::Medium), 5000);
    }

    #[test]
    fn max_score_hard_is_5000() {
        assert_eq!(max_score(&GameMode::Hard), 5000);
    }
}
