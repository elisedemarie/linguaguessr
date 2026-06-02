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

pub fn round_result_emoji(score: u32) -> &'static str {
    match score {
        1000 => "🟩",
        0    => "🟥",
        _    => "🟨",
    }
}

pub fn sort_scores_descending(mut scores: Vec<u32>) -> Vec<u32> {
    scores.sort_unstable_by(|a, b| b.cmp(a));
    scores
}

pub fn format_share_text(date: &str, emojis: &str, total_score: u32) -> String {
    format!("LinguaGuessr Daily — {date}\n{emojis}\n{total_score} / 5000\nlinguaguessr.io")
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

    #[test]
    fn display_score_daily_returns_raw() {
        assert_eq!(display_score(3500, &GameMode::Daily), 3500);
    }

    #[test]
    fn max_score_daily_is_5000() {
        assert_eq!(max_score(&GameMode::Daily), 5000);
    }

    // --- round_result_emoji ---

    #[test]
    fn perfect_score_is_green() {
        assert_eq!(round_result_emoji(1000), "🟩");
    }

    #[test]
    fn zero_score_is_red() {
        assert_eq!(round_result_emoji(0), "🟥");
    }

    #[test]
    fn partial_score_is_yellow() {
        assert_eq!(round_result_emoji(500), "🟨");
    }

    #[test]
    fn minimal_partial_score_is_yellow() {
        assert_eq!(round_result_emoji(1), "🟨");
    }

    #[test]
    fn just_below_perfect_is_yellow() {
        assert_eq!(round_result_emoji(999), "🟨");
    }

    // --- sort_scores_descending ---

    #[test]
    fn sort_scores_descending_highest_first() {
        assert_eq!(sort_scores_descending(vec![1000, 4000, 2000]), vec![4000, 2000, 1000]);
    }

    #[test]
    fn sort_scores_descending_already_sorted_unchanged() {
        assert_eq!(sort_scores_descending(vec![5000, 3000, 1000]), vec![5000, 3000, 1000]);
    }

    #[test]
    fn sort_scores_descending_empty_returns_empty() {
        assert_eq!(sort_scores_descending(vec![]), Vec::<u32>::new());
    }

    #[test]
    fn sort_scores_descending_single_element() {
        assert_eq!(sort_scores_descending(vec![2500]), vec![2500]);
    }

    #[test]
    fn sort_scores_descending_equal_scores_preserved() {
        assert_eq!(sort_scores_descending(vec![1000, 1000, 1000]), vec![1000, 1000, 1000]);
    }

    // --- format_share_text ---

    #[test]
    fn share_text_contains_date() {
        let text = format_share_text("2026-05-30", "🟩🟨🟥🟩🟩", 3500);
        assert!(text.contains("2026-05-30"));
    }

    #[test]
    fn share_text_contains_emojis() {
        let text = format_share_text("2026-05-30", "🟩🟨🟥🟩🟩", 3500);
        assert!(text.contains("🟩🟨🟥🟩🟩"));
    }

    #[test]
    fn share_text_contains_total_score() {
        let text = format_share_text("2026-05-30", "🟩🟨🟥🟩🟩", 3500);
        assert!(text.contains("3500"));
    }

    #[test]
    fn share_text_contains_max_score() {
        let text = format_share_text("2026-05-30", "🟩🟨🟥🟩🟩", 3500);
        assert!(text.contains("5000"));
    }

    #[test]
    fn share_text_contains_site_url() {
        let text = format_share_text("2026-05-30", "🟩🟨🟥🟩🟩", 3500);
        assert!(text.contains("linguaguessr.io"));
    }
}
