use serde::{Deserialize, Serialize};

pub const STORAGE_KEY: &str = "linguaguessr_daily";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyEntry {
    pub date:   String,
    pub emojis: String,
    pub score:  u32,
}

pub fn daily_already_played(entry: &DailyEntry, today: &str) -> bool {
    entry.date == today
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(date: &str) -> DailyEntry {
        DailyEntry { date: date.into(), emojis: "🟩🟨🟥🟩🟩".into(), score: 3500 }
    }

    #[test]
    fn returns_true_when_date_matches_today() {
        assert!(daily_already_played(&entry("2026-05-30"), "2026-05-30"));
    }

    #[test]
    fn returns_false_when_date_is_yesterday() {
        assert!(!daily_already_played(&entry("2026-05-29"), "2026-05-30"));
    }

    #[test]
    fn returns_false_when_date_is_tomorrow() {
        assert!(!daily_already_played(&entry("2026-05-31"), "2026-05-30"));
    }

    #[test]
    fn returns_false_when_entry_is_from_different_month() {
        assert!(!daily_already_played(&entry("2026-04-30"), "2026-05-30"));
    }
}
