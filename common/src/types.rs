use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    French,
    Japanese,
    Arabic,
    Russian,
}

impl Language {
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::French,
            Language::Japanese,
            Language::Arabic,
            Language::Russian,
        ]
    }

    fn aliases(&self) -> &[&str] {
        match self {
            Language::English => &["en", "english"],
            Language::French => &["fr", "french", "français", "francais"],
            Language::Japanese => &["ja", "japanese", "日本語"],
            Language::Arabic => &["ar", "arabic", "العربية"],
            Language::Russian => &["ru", "russian", "русский"],
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Language::English => "English (EN)",
            Language::French => "French (FR)",
            Language::Japanese => "Japanese (JA)",
            Language::Arabic => "Arabic (AR)",
            Language::Russian => "Russian (RU)",
        }
    }

    pub fn suggestions(query: &str) -> Vec<Language> {
        if query.is_empty() {
            return Language::all().to_vec();
        }
        let query_lower = query.to_lowercase();
        Language::all()
            .iter()
            .filter(|lang| {
                lang.aliases()
                    .iter()
                    .any(|alias| alias.starts_with(query_lower.as_str()))
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- empty query ---

    #[test]
    fn empty_query_returns_all_five_languages() {
        let result = Language::suggestions("");
        assert_eq!(result.len(), 5);
        assert!(result.contains(&Language::English));
        assert!(result.contains(&Language::French));
        assert!(result.contains(&Language::Japanese));
        assert!(result.contains(&Language::Arabic));
        assert!(result.contains(&Language::Russian));
    }

    // --- English aliases ---

    #[test]
    fn en_alias_returns_english() {
        assert_eq!(Language::suggestions("en"), vec![Language::English]);
    }

    #[test]
    fn english_alias_returns_english() {
        assert_eq!(Language::suggestions("english"), vec![Language::English]);
    }

    // --- French aliases ---

    #[test]
    fn fr_alias_returns_french() {
        assert_eq!(Language::suggestions("fr"), vec![Language::French]);
    }

    #[test]
    fn french_alias_returns_french() {
        assert_eq!(Language::suggestions("french"), vec![Language::French]);
    }

    #[test]
    fn francais_alias_returns_french() {
        assert_eq!(Language::suggestions("francais"), vec![Language::French]);
    }

    #[test]
    fn francais_with_cedilla_returns_french() {
        assert_eq!(Language::suggestions("français"), vec![Language::French]);
    }

    // --- Japanese aliases ---

    #[test]
    fn ja_alias_returns_japanese() {
        assert_eq!(Language::suggestions("ja"), vec![Language::Japanese]);
    }

    #[test]
    fn japanese_alias_returns_japanese() {
        assert_eq!(Language::suggestions("japanese"), vec![Language::Japanese]);
    }

    #[test]
    fn japanese_script_prefix_returns_japanese() {
        assert_eq!(Language::suggestions("日"), vec![Language::Japanese]);
    }

    // --- Arabic aliases ---

    #[test]
    fn ar_alias_returns_arabic() {
        assert_eq!(Language::suggestions("ar"), vec![Language::Arabic]);
    }

    #[test]
    fn arabic_alias_returns_arabic() {
        assert_eq!(Language::suggestions("arabic"), vec![Language::Arabic]);
    }

    #[test]
    fn arabic_script_prefix_returns_arabic() {
        assert_eq!(Language::suggestions("الع"), vec![Language::Arabic]);
    }

    // --- Russian aliases ---

    #[test]
    fn ru_alias_returns_russian() {
        assert_eq!(Language::suggestions("ru"), vec![Language::Russian]);
    }

    #[test]
    fn russian_alias_returns_russian() {
        assert_eq!(Language::suggestions("russian"), vec![Language::Russian]);
    }

    #[test]
    fn russian_script_prefix_returns_russian() {
        assert_eq!(Language::suggestions("рус"), vec![Language::Russian]);
    }

    // --- case insensitivity ---

    #[test]
    fn uppercase_fr_returns_french() {
        assert_eq!(Language::suggestions("FR"), vec![Language::French]);
    }

    #[test]
    fn uppercase_english_returns_english() {
        assert_eq!(Language::suggestions("ENGLISH"), vec![Language::English]);
    }

    #[test]
    fn mixed_case_ru_returns_russian() {
        assert_eq!(Language::suggestions("Ru"), vec![Language::Russian]);
    }

    // --- starts_with, not contains ---

    #[test]
    fn mid_word_query_returns_empty() {
        assert_eq!(Language::suggestions("ussian"), vec![]);
    }

    #[test]
    fn mid_word_english_query_returns_empty() {
        assert_eq!(Language::suggestions("nglish"), vec![]);
    }

    // --- unrecognised ---

    #[test]
    fn unrecognised_query_returns_empty() {
        assert_eq!(Language::suggestions("klingon"), vec![]);
    }

    // --- label ---

    #[test]
    fn english_label() {
        assert_eq!(Language::English.label(), "English (EN)");
    }

    #[test]
    fn french_label() {
        assert_eq!(Language::French.label(), "French (FR)");
    }

    #[test]
    fn japanese_label() {
        assert_eq!(Language::Japanese.label(), "Japanese (JA)");
    }

    #[test]
    fn arabic_label() {
        assert_eq!(Language::Arabic.label(), "Arabic (AR)");
    }

    #[test]
    fn russian_label() {
        assert_eq!(Language::Russian.label(), "Russian (RU)");
    }
}
