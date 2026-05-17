use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    French,
    Japanese,
    Arabic,
    Russian,
    Spanish,
    Chinese,
    Hindi,
    Bengali,
    Portuguese,
    Indonesian,
    Urdu,
    German,
    Korean,
    Vietnamese,
    Telugu,
    Marathi,
    Tamil,
    Turkish,
    Persian,
    Italian,
    Thai,
    Swahili,
    Polish,
    Ukrainian,
    Dutch,
    Greek,
    Romanian,
    Czech,
    Hungarian,
}

impl Language {
    pub fn all() -> &'static [Language] {
        &[
            Language::English,
            Language::French,
            Language::Japanese,
            Language::Arabic,
            Language::Russian,
            Language::Spanish,
            Language::Chinese,
            Language::Hindi,
            Language::Bengali,
            Language::Portuguese,
            Language::Indonesian,
            Language::Urdu,
            Language::German,
            Language::Korean,
            Language::Vietnamese,
            Language::Telugu,
            Language::Marathi,
            Language::Tamil,
            Language::Turkish,
            Language::Persian,
            Language::Italian,
            Language::Thai,
            Language::Swahili,
            Language::Polish,
            Language::Ukrainian,
            Language::Dutch,
            Language::Greek,
            Language::Romanian,
            Language::Czech,
            Language::Hungarian,
        ]
    }

    fn aliases(&self) -> &[&str] {
        match self {
            Language::English    => &["en", "english"],
            Language::French     => &["fr", "french", "français", "francais"],
            Language::Japanese   => &["ja", "japanese", "日本語"],
            Language::Arabic     => &["ar", "arabic", "العربية"],
            Language::Russian    => &["ru", "russian", "русский"],
            Language::Spanish    => &["es", "spanish", "español", "espanol"],
            Language::Chinese    => &["zh", "chinese", "mandarin", "中文", "普通话"],
            Language::Hindi      => &["hi", "hindi", "हिंदी"],
            Language::Bengali    => &["bn", "bengali", "bangla", "বাংলা"],
            Language::Portuguese => &["pt", "portuguese", "português", "portugues"],
            Language::Indonesian => &["id", "indonesian", "bahasa"],
            Language::Urdu       => &["ur", "urdu", "اردو"],
            Language::German     => &["de", "german", "deutsch"],
            Language::Korean     => &["ko", "korean", "한국어"],
            Language::Vietnamese => &["vi", "vietnamese"],
            Language::Telugu     => &["te", "telugu", "తెలుగు"],
            Language::Marathi    => &["mr", "marathi", "मराठी"],
            Language::Tamil      => &["ta", "tamil", "தமிழ்"],
            Language::Turkish    => &["tr", "turkish", "türkçe", "turkce"],
            Language::Persian    => &["fa", "persian", "farsi", "فارسی"],
            Language::Italian    => &["it", "italian", "italiano"],
            Language::Thai       => &["th", "thai", "ภาษาไทย"],
            Language::Swahili    => &["sw", "swahili", "kiswahili"],
            Language::Polish     => &["pl", "polish", "polski"],
            Language::Ukrainian  => &["uk", "ukrainian", "українська"],
            Language::Dutch      => &["nl", "dutch", "nederlands"],
            Language::Greek      => &["el", "greek", "ελληνικά"],
            Language::Romanian   => &["ro", "romanian", "română", "romana"],
            Language::Czech      => &["cs", "czech", "čeština", "cestina"],
            Language::Hungarian  => &["hu", "hungarian", "magyar"],
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Language::English    => "English (EN)",
            Language::French     => "French (FR)",
            Language::Japanese   => "Japanese (JA)",
            Language::Arabic     => "Arabic (AR)",
            Language::Russian    => "Russian (RU)",
            Language::Spanish    => "Spanish (ES)",
            Language::Chinese    => "Chinese (ZH)",
            Language::Hindi      => "Hindi (HI)",
            Language::Bengali    => "Bengali (BN)",
            Language::Portuguese => "Portuguese (PT)",
            Language::Indonesian => "Indonesian (ID)",
            Language::Urdu       => "Urdu (UR)",
            Language::German     => "German (DE)",
            Language::Korean     => "Korean (KO)",
            Language::Vietnamese => "Vietnamese (VI)",
            Language::Telugu     => "Telugu (TE)",
            Language::Marathi    => "Marathi (MR)",
            Language::Tamil      => "Tamil (TA)",
            Language::Turkish    => "Turkish (TR)",
            Language::Persian    => "Persian (FA)",
            Language::Italian    => "Italian (IT)",
            Language::Thai       => "Thai (TH)",
            Language::Swahili    => "Swahili (SW)",
            Language::Polish     => "Polish (PL)",
            Language::Ukrainian  => "Ukrainian (UK)",
            Language::Dutch      => "Dutch (NL)",
            Language::Greek      => "Greek (EL)",
            Language::Romanian   => "Romanian (RO)",
            Language::Czech      => "Czech (CS)",
            Language::Hungarian  => "Hungarian (HU)",
        }
    }

    pub fn suggestions(query: &str) -> Vec<Language> {
        let query_lower = query.to_lowercase();
        let mut results: Vec<Language> = Language::all()
            .iter()
            .filter(|lang| {
                query_lower.is_empty() || lang.aliases()
                    .iter()
                    .any(|alias| alias.starts_with(query_lower.as_str()))
            })
            .cloned()
            .collect();
        results.sort_by(|a, b| a.label().cmp(b.label()));
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- empty query ---

    #[test]
    fn empty_query_returns_all_languages() {
        let result = Language::suggestions("");
        assert_eq!(result.len(), 30);
    }

    #[test]
    fn empty_query_first_result_is_arabic() {
        let result = Language::suggestions("");
        assert_eq!(result[0], Language::Arabic);
    }

    #[test]
    fn empty_query_last_result_is_vietnamese() {
        let result = Language::suggestions("");
        assert_eq!(result[result.len() - 1], Language::Vietnamese);
    }

    #[test]
    fn filtered_results_are_also_sorted() {
        let result = Language::suggestions("p");
        // Persian (FA), Polish (PL), Portuguese (PT) — alphabetical
        assert_eq!(result, vec![Language::Persian, Language::Polish, Language::Portuguese]);
    }

    // --- original 5 languages ---

    #[test]
    fn en_alias_returns_english() {
        assert_eq!(Language::suggestions("en"), vec![Language::English]);
    }

    #[test]
    fn english_alias_returns_english() {
        assert_eq!(Language::suggestions("english"), vec![Language::English]);
    }

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

    // --- new languages: code alias ---

    #[test]
    fn es_alias_returns_spanish() {
        assert_eq!(Language::suggestions("es"), vec![Language::Spanish]);
    }

    #[test]
    fn spanish_alias_returns_spanish() {
        assert_eq!(Language::suggestions("spanish"), vec![Language::Spanish]);
    }

    #[test]
    fn zh_alias_returns_chinese() {
        assert_eq!(Language::suggestions("zh"), vec![Language::Chinese]);
    }

    #[test]
    fn mandarin_alias_returns_chinese() {
        assert_eq!(Language::suggestions("mandarin"), vec![Language::Chinese]);
    }

    #[test]
    fn hi_alias_returns_hindi() {
        assert_eq!(Language::suggestions("hi"), vec![Language::Hindi]);
    }

    #[test]
    fn hindi_alias_returns_hindi() {
        assert_eq!(Language::suggestions("hindi"), vec![Language::Hindi]);
    }

    #[test]
    fn bn_alias_returns_bengali() {
        assert_eq!(Language::suggestions("bn"), vec![Language::Bengali]);
    }

    #[test]
    fn bengali_alias_returns_bengali() {
        assert_eq!(Language::suggestions("bengali"), vec![Language::Bengali]);
    }

    #[test]
    fn pt_alias_returns_portuguese() {
        assert_eq!(Language::suggestions("pt"), vec![Language::Portuguese]);
    }

    #[test]
    fn portuguese_alias_returns_portuguese() {
        assert_eq!(Language::suggestions("portuguese"), vec![Language::Portuguese]);
    }

    #[test]
    fn id_alias_returns_indonesian() {
        assert_eq!(Language::suggestions("id"), vec![Language::Indonesian]);
    }

    #[test]
    fn indonesian_alias_returns_indonesian() {
        assert_eq!(Language::suggestions("indonesian"), vec![Language::Indonesian]);
    }

    #[test]
    fn ur_alias_returns_urdu() {
        assert_eq!(Language::suggestions("ur"), vec![Language::Urdu]);
    }

    #[test]
    fn urdu_alias_returns_urdu() {
        assert_eq!(Language::suggestions("urdu"), vec![Language::Urdu]);
    }

    #[test]
    fn de_alias_returns_german() {
        assert_eq!(Language::suggestions("de"), vec![Language::German]);
    }

    #[test]
    fn german_alias_returns_german() {
        assert_eq!(Language::suggestions("german"), vec![Language::German]);
    }

    #[test]
    fn ko_alias_returns_korean() {
        assert_eq!(Language::suggestions("ko"), vec![Language::Korean]);
    }

    #[test]
    fn korean_alias_returns_korean() {
        assert_eq!(Language::suggestions("korean"), vec![Language::Korean]);
    }

    #[test]
    fn vi_alias_returns_vietnamese() {
        assert_eq!(Language::suggestions("vi"), vec![Language::Vietnamese]);
    }

    #[test]
    fn vietnamese_alias_returns_vietnamese() {
        assert_eq!(Language::suggestions("vietnamese"), vec![Language::Vietnamese]);
    }

    #[test]
    fn te_alias_returns_telugu() {
        assert_eq!(Language::suggestions("te"), vec![Language::Telugu]);
    }

    #[test]
    fn telugu_alias_returns_telugu() {
        assert_eq!(Language::suggestions("telugu"), vec![Language::Telugu]);
    }

    #[test]
    fn mr_alias_returns_marathi() {
        assert_eq!(Language::suggestions("mr"), vec![Language::Marathi]);
    }

    #[test]
    fn marathi_alias_returns_marathi() {
        assert_eq!(Language::suggestions("marathi"), vec![Language::Marathi]);
    }

    #[test]
    fn ta_alias_returns_tamil() {
        assert_eq!(Language::suggestions("ta"), vec![Language::Tamil]);
    }

    #[test]
    fn tamil_alias_returns_tamil() {
        assert_eq!(Language::suggestions("tamil"), vec![Language::Tamil]);
    }

    #[test]
    fn tr_alias_returns_turkish() {
        assert_eq!(Language::suggestions("tr"), vec![Language::Turkish]);
    }

    #[test]
    fn turkish_alias_returns_turkish() {
        assert_eq!(Language::suggestions("turkish"), vec![Language::Turkish]);
    }

    #[test]
    fn fa_alias_returns_persian() {
        assert_eq!(Language::suggestions("fa"), vec![Language::Persian]);
    }

    #[test]
    fn farsi_alias_returns_persian() {
        assert_eq!(Language::suggestions("farsi"), vec![Language::Persian]);
    }

    #[test]
    fn it_alias_returns_italian() {
        assert_eq!(Language::suggestions("it"), vec![Language::Italian]);
    }

    #[test]
    fn italian_alias_returns_italian() {
        assert_eq!(Language::suggestions("italian"), vec![Language::Italian]);
    }

    #[test]
    fn th_alias_returns_thai() {
        assert_eq!(Language::suggestions("th"), vec![Language::Thai]);
    }

    #[test]
    fn thai_alias_returns_thai() {
        assert_eq!(Language::suggestions("thai"), vec![Language::Thai]);
    }

    #[test]
    fn sw_alias_returns_swahili() {
        assert_eq!(Language::suggestions("sw"), vec![Language::Swahili]);
    }

    #[test]
    fn swahili_alias_returns_swahili() {
        assert_eq!(Language::suggestions("swahili"), vec![Language::Swahili]);
    }

    #[test]
    fn pl_alias_returns_polish() {
        assert_eq!(Language::suggestions("pl"), vec![Language::Polish]);
    }

    #[test]
    fn polish_alias_returns_polish() {
        assert_eq!(Language::suggestions("polish"), vec![Language::Polish]);
    }

    #[test]
    fn uk_alias_returns_ukrainian() {
        assert_eq!(Language::suggestions("uk"), vec![Language::Ukrainian]);
    }

    #[test]
    fn ukrainian_alias_returns_ukrainian() {
        assert_eq!(Language::suggestions("ukrainian"), vec![Language::Ukrainian]);
    }

    #[test]
    fn nl_alias_returns_dutch() {
        assert_eq!(Language::suggestions("nl"), vec![Language::Dutch]);
    }

    #[test]
    fn dutch_alias_returns_dutch() {
        assert_eq!(Language::suggestions("dutch"), vec![Language::Dutch]);
    }

    #[test]
    fn el_alias_returns_greek() {
        assert_eq!(Language::suggestions("el"), vec![Language::Greek]);
    }

    #[test]
    fn greek_alias_returns_greek() {
        assert_eq!(Language::suggestions("greek"), vec![Language::Greek]);
    }

    #[test]
    fn ro_alias_returns_romanian() {
        assert_eq!(Language::suggestions("ro"), vec![Language::Romanian]);
    }

    #[test]
    fn romanian_alias_returns_romanian() {
        assert_eq!(Language::suggestions("romanian"), vec![Language::Romanian]);
    }

    #[test]
    fn cs_alias_returns_czech() {
        assert_eq!(Language::suggestions("cs"), vec![Language::Czech]);
    }

    #[test]
    fn czech_alias_returns_czech() {
        assert_eq!(Language::suggestions("czech"), vec![Language::Czech]);
    }

    #[test]
    fn hu_alias_returns_hungarian() {
        assert_eq!(Language::suggestions("hu"), vec![Language::Hungarian]);
    }

    #[test]
    fn hungarian_alias_returns_hungarian() {
        assert_eq!(Language::suggestions("hungarian"), vec![Language::Hungarian]);
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
    fn uppercase_es_returns_spanish() {
        assert_eq!(Language::suggestions("ES"), vec![Language::Spanish]);
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

    // --- labels ---

    #[test]
    fn english_label() { assert_eq!(Language::English.label(), "English (EN)"); }
    #[test]
    fn french_label() { assert_eq!(Language::French.label(), "French (FR)"); }
    #[test]
    fn japanese_label() { assert_eq!(Language::Japanese.label(), "Japanese (JA)"); }
    #[test]
    fn arabic_label() { assert_eq!(Language::Arabic.label(), "Arabic (AR)"); }
    #[test]
    fn russian_label() { assert_eq!(Language::Russian.label(), "Russian (RU)"); }
    #[test]
    fn spanish_label() { assert_eq!(Language::Spanish.label(), "Spanish (ES)"); }
    #[test]
    fn chinese_label() { assert_eq!(Language::Chinese.label(), "Chinese (ZH)"); }
    #[test]
    fn hindi_label() { assert_eq!(Language::Hindi.label(), "Hindi (HI)"); }
    #[test]
    fn bengali_label() { assert_eq!(Language::Bengali.label(), "Bengali (BN)"); }
    #[test]
    fn portuguese_label() { assert_eq!(Language::Portuguese.label(), "Portuguese (PT)"); }
    #[test]
    fn indonesian_label() { assert_eq!(Language::Indonesian.label(), "Indonesian (ID)"); }
    #[test]
    fn urdu_label() { assert_eq!(Language::Urdu.label(), "Urdu (UR)"); }
    #[test]
    fn german_label() { assert_eq!(Language::German.label(), "German (DE)"); }
    #[test]
    fn korean_label() { assert_eq!(Language::Korean.label(), "Korean (KO)"); }
    #[test]
    fn vietnamese_label() { assert_eq!(Language::Vietnamese.label(), "Vietnamese (VI)"); }
    #[test]
    fn telugu_label() { assert_eq!(Language::Telugu.label(), "Telugu (TE)"); }
    #[test]
    fn marathi_label() { assert_eq!(Language::Marathi.label(), "Marathi (MR)"); }
    #[test]
    fn tamil_label() { assert_eq!(Language::Tamil.label(), "Tamil (TA)"); }
    #[test]
    fn turkish_label() { assert_eq!(Language::Turkish.label(), "Turkish (TR)"); }
    #[test]
    fn persian_label() { assert_eq!(Language::Persian.label(), "Persian (FA)"); }
    #[test]
    fn italian_label() { assert_eq!(Language::Italian.label(), "Italian (IT)"); }
    #[test]
    fn thai_label() { assert_eq!(Language::Thai.label(), "Thai (TH)"); }
    #[test]
    fn swahili_label() { assert_eq!(Language::Swahili.label(), "Swahili (SW)"); }
    #[test]
    fn polish_label() { assert_eq!(Language::Polish.label(), "Polish (PL)"); }
    #[test]
    fn ukrainian_label() { assert_eq!(Language::Ukrainian.label(), "Ukrainian (UK)"); }
    #[test]
    fn dutch_label() { assert_eq!(Language::Dutch.label(), "Dutch (NL)"); }
    #[test]
    fn greek_label() { assert_eq!(Language::Greek.label(), "Greek (EL)"); }
    #[test]
    fn romanian_label() { assert_eq!(Language::Romanian.label(), "Romanian (RO)"); }
    #[test]
    fn czech_label() { assert_eq!(Language::Czech.label(), "Czech (CS)"); }
    #[test]
    fn hungarian_label() { assert_eq!(Language::Hungarian.label(), "Hungarian (HU)"); }
}
