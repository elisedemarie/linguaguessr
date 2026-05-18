use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum GameMode {
    Easy,
    #[default]
    Medium,
    Hard,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    // original 30
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
    // new 45
    Afrikaans,
    Albanian,
    Amharic,
    Armenian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bulgarian,
    Burmese,
    Catalan,
    Croatian,
    Danish,
    Estonian,
    Finnish,
    Galician,
    Georgian,
    Gujarati,
    Hausa,
    Hebrew,
    Icelandic,
    Irish,
    Kannada,
    Kazakh,
    Khmer,
    Latvian,
    Lithuanian,
    Macedonian,
    Malay,
    Malayalam,
    Mongolian,
    Nepali,
    Norwegian,
    Odia,
    Punjabi,
    Serbian,
    Sinhala,
    Slovak,
    Slovenian,
    Somali,
    Swedish,
    Tagalog,
    Uzbek,
    Welsh,
    Yoruba,
    Zulu,
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
            Language::Afrikaans,
            Language::Albanian,
            Language::Amharic,
            Language::Armenian,
            Language::Azerbaijani,
            Language::Basque,
            Language::Belarusian,
            Language::Bulgarian,
            Language::Burmese,
            Language::Catalan,
            Language::Croatian,
            Language::Danish,
            Language::Estonian,
            Language::Finnish,
            Language::Galician,
            Language::Georgian,
            Language::Gujarati,
            Language::Hausa,
            Language::Hebrew,
            Language::Icelandic,
            Language::Irish,
            Language::Kannada,
            Language::Kazakh,
            Language::Khmer,
            Language::Latvian,
            Language::Lithuanian,
            Language::Macedonian,
            Language::Malay,
            Language::Malayalam,
            Language::Mongolian,
            Language::Nepali,
            Language::Norwegian,
            Language::Odia,
            Language::Punjabi,
            Language::Serbian,
            Language::Sinhala,
            Language::Slovak,
            Language::Slovenian,
            Language::Somali,
            Language::Swedish,
            Language::Tagalog,
            Language::Uzbek,
            Language::Welsh,
            Language::Yoruba,
            Language::Zulu,
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
            Language::Afrikaans  => &["af", "afrikaans"],
            Language::Albanian   => &["sq", "albanian", "shqip"],
            Language::Amharic    => &["am", "amharic", "አማርኛ"],
            Language::Armenian   => &["hy", "armenian", "հայերեն"],
            Language::Azerbaijani => &["az", "azerbaijani", "azərbaycan"],
            Language::Basque     => &["eu", "basque", "euskera"],
            Language::Belarusian => &["be", "belarusian", "беларуская"],
            Language::Bulgarian  => &["bg", "bulgarian", "български"],
            Language::Burmese    => &["my", "burmese", "myanmar", "မြန်မာဘာသာ"],
            Language::Catalan    => &["ca", "catalan", "català"],
            Language::Croatian   => &["hr", "croatian", "hrvatski"],
            Language::Danish     => &["da", "danish", "dansk"],
            Language::Estonian   => &["et", "estonian", "eesti"],
            Language::Finnish    => &["fi", "finnish", "suomi"],
            Language::Galician   => &["gl", "galician", "galego"],
            Language::Georgian   => &["ka", "georgian", "ქართული"],
            Language::Gujarati   => &["gu", "gujarati", "ગુજરાતી"],
            Language::Hausa      => &["ha", "hausa"],
            Language::Hebrew     => &["he", "hebrew", "עברית"],
            Language::Icelandic  => &["is", "icelandic", "íslenska"],
            Language::Irish      => &["ga", "irish", "gaeilge"],
            Language::Kannada    => &["kn", "kannada", "ಕನ್ನಡ"],
            Language::Kazakh     => &["kk", "kazakh", "қазақ"],
            Language::Khmer      => &["km", "khmer", "ខ្មែរ"],
            Language::Latvian    => &["lv", "latvian", "latviešu"],
            Language::Lithuanian => &["lt", "lithuanian", "lietuvių"],
            Language::Macedonian => &["mk", "macedonian", "македонски"],
            Language::Malay      => &["ms", "malay", "melayu"],
            Language::Malayalam  => &["ml", "malayalam", "മലയാളം"],
            Language::Mongolian  => &["mn", "mongolian", "монгол"],
            Language::Nepali     => &["ne", "nepali", "नेपाली"],
            Language::Norwegian  => &["no", "norwegian", "norsk"],
            Language::Odia       => &["or", "odia", "oriya", "ଓଡ଼ିଆ"],
            Language::Punjabi    => &["pa", "punjabi", "ਪੰਜਾਬੀ"],
            Language::Serbian    => &["sr", "serbian", "српски"],
            Language::Sinhala    => &["si", "sinhala", "sinhalese", "සිංහල"],
            Language::Slovak     => &["sk", "slovak", "slovenčina"],
            Language::Slovenian  => &["sl", "slovenian", "slovenščina"],
            Language::Somali     => &["so", "somali", "soomaali"],
            Language::Swedish    => &["sv", "swedish", "svenska"],
            Language::Tagalog    => &["tl", "tagalog", "filipino"],
            Language::Uzbek      => &["uz", "uzbek"],
            Language::Welsh      => &["cy", "welsh", "cymraeg"],
            Language::Yoruba     => &["yo", "yoruba"],
            Language::Zulu       => &["zu", "zulu"],
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
            Language::Afrikaans  => "Afrikaans (AF)",
            Language::Albanian   => "Albanian (SQ)",
            Language::Amharic    => "Amharic (AM)",
            Language::Armenian   => "Armenian (HY)",
            Language::Azerbaijani => "Azerbaijani (AZ)",
            Language::Basque     => "Basque (EU)",
            Language::Belarusian => "Belarusian (BE)",
            Language::Bulgarian  => "Bulgarian (BG)",
            Language::Burmese    => "Burmese (MY)",
            Language::Catalan    => "Catalan (CA)",
            Language::Croatian   => "Croatian (HR)",
            Language::Danish     => "Danish (DA)",
            Language::Estonian   => "Estonian (ET)",
            Language::Finnish    => "Finnish (FI)",
            Language::Galician   => "Galician (GL)",
            Language::Georgian   => "Georgian (KA)",
            Language::Gujarati   => "Gujarati (GU)",
            Language::Hausa      => "Hausa (HA)",
            Language::Hebrew     => "Hebrew (HE)",
            Language::Icelandic  => "Icelandic (IS)",
            Language::Irish      => "Irish (GA)",
            Language::Kannada    => "Kannada (KN)",
            Language::Kazakh     => "Kazakh (KK)",
            Language::Khmer      => "Khmer (KM)",
            Language::Latvian    => "Latvian (LV)",
            Language::Lithuanian => "Lithuanian (LT)",
            Language::Macedonian => "Macedonian (MK)",
            Language::Malay      => "Malay (MS)",
            Language::Malayalam  => "Malayalam (ML)",
            Language::Mongolian  => "Mongolian (MN)",
            Language::Nepali     => "Nepali (NE)",
            Language::Norwegian  => "Norwegian (NO)",
            Language::Odia       => "Odia (OR)",
            Language::Punjabi    => "Punjabi (PA)",
            Language::Serbian    => "Serbian (SR)",
            Language::Sinhala    => "Sinhala (SI)",
            Language::Slovak     => "Slovak (SK)",
            Language::Slovenian  => "Slovenian (SL)",
            Language::Somali     => "Somali (SO)",
            Language::Swedish    => "Swedish (SV)",
            Language::Tagalog    => "Tagalog (TL)",
            Language::Uzbek      => "Uzbek (UZ)",
            Language::Welsh      => "Welsh (CY)",
            Language::Yoruba     => "Yoruba (YO)",
            Language::Zulu       => "Zulu (ZU)",
        }
    }

    pub fn medium_pool() -> &'static [Language] {
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

    pub fn easy_pool() -> &'static [Language] {
        &[
            Language::English,
            Language::Chinese,
            Language::Hindi,
            Language::Spanish,
            Language::French,
            Language::Arabic,
            Language::Bengali,
            Language::Portuguese,
            Language::Russian,
            Language::Urdu,
        ]
    }

    pub fn suggestions_for(query: &str, pool: &[Language]) -> Vec<Language> {
        let query_lower = query.to_lowercase();
        let mut results: Vec<Language> = pool
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

    pub fn suggestions(query: &str) -> Vec<Language> {
        Self::suggestions_for(query, Self::all())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- GameMode ---

    #[test]
    fn game_mode_default_is_medium() {
        assert_eq!(GameMode::default(), GameMode::Medium);
    }

    #[test]
    fn game_mode_easy_deserialises_from_lowercase() {
        let mode: GameMode = serde_json::from_str(r#""easy""#).unwrap();
        assert_eq!(mode, GameMode::Easy);
    }

    #[test]
    fn game_mode_medium_deserialises_from_lowercase() {
        let mode: GameMode = serde_json::from_str(r#""medium""#).unwrap();
        assert_eq!(mode, GameMode::Medium);
    }

    #[test]
    fn game_mode_hard_deserialises_from_lowercase() {
        let mode: GameMode = serde_json::from_str(r#""hard""#).unwrap();
        assert_eq!(mode, GameMode::Hard);
    }

    #[test]
    fn game_mode_easy_serialises_as_lowercase() {
        assert_eq!(serde_json::to_string(&GameMode::Easy).unwrap(), r#""easy""#);
    }

    #[test]
    fn game_mode_hard_serialises_as_lowercase() {
        assert_eq!(serde_json::to_string(&GameMode::Hard).unwrap(), r#""hard""#);
    }

    // --- medium_pool ---

    #[test]
    fn medium_pool_returns_exactly_30_languages() {
        assert_eq!(Language::medium_pool().len(), 30);
    }

    #[test]
    fn medium_pool_contains_english() {
        assert!(Language::medium_pool().contains(&Language::English));
    }

    #[test]
    fn medium_pool_contains_hungarian() {
        assert!(Language::medium_pool().contains(&Language::Hungarian));
    }

    #[test]
    fn medium_pool_does_not_contain_afrikaans() {
        assert!(!Language::medium_pool().contains(&Language::Afrikaans));
    }

    #[test]
    fn medium_pool_does_not_contain_swedish() {
        assert!(!Language::medium_pool().contains(&Language::Swedish));
    }

    // --- easy_pool ---

    #[test]
    fn easy_pool_returns_exactly_10_languages() {
        assert_eq!(Language::easy_pool().len(), 10);
    }

    #[test]
    fn easy_pool_contains_the_10_most_spoken_languages() {
        let pool = Language::easy_pool();
        assert!(pool.contains(&Language::English));
        assert!(pool.contains(&Language::Chinese));
        assert!(pool.contains(&Language::Hindi));
        assert!(pool.contains(&Language::Spanish));
        assert!(pool.contains(&Language::French));
        assert!(pool.contains(&Language::Arabic));
        assert!(pool.contains(&Language::Bengali));
        assert!(pool.contains(&Language::Portuguese));
        assert!(pool.contains(&Language::Russian));
        assert!(pool.contains(&Language::Urdu));
    }

    #[test]
    fn easy_pool_does_not_contain_japanese() {
        assert!(!Language::easy_pool().contains(&Language::Japanese));
    }

    // --- suggestions_for ---

    #[test]
    fn suggestions_for_empty_query_returns_full_pool() {
        assert_eq!(Language::suggestions_for("", Language::medium_pool()).len(), 30);
    }

    #[test]
    fn suggestions_for_filters_within_pool() {
        let result = Language::suggestions_for("en", Language::medium_pool());
        assert_eq!(result, vec![Language::English]);
    }

    #[test]
    fn suggestions_for_excludes_languages_outside_pool() {
        // Swedish is in all() but not in medium_pool()
        let result = Language::suggestions_for("sw", Language::medium_pool());
        assert_eq!(result, vec![Language::Swahili]);
    }

    #[test]
    fn suggestions_for_returns_sorted_results() {
        let result = Language::suggestions_for("p", Language::medium_pool());
        assert_eq!(result, vec![Language::Persian, Language::Polish, Language::Portuguese]);
    }

    // --- empty query ---

    #[test]
    fn empty_query_returns_all_languages() {
        let result = Language::suggestions("");
        assert_eq!(result.len(), 75);
    }

    #[test]
    fn empty_query_first_result_is_afrikaans() {
        let result = Language::suggestions("");
        assert_eq!(result[0], Language::Afrikaans);
    }

    #[test]
    fn empty_query_last_result_is_zulu() {
        let result = Language::suggestions("");
        assert_eq!(result[result.len() - 1], Language::Zulu);
    }

    #[test]
    fn filtered_results_are_also_sorted() {
        let result = Language::suggestions("p");
        assert_eq!(result, vec![
            Language::Persian,
            Language::Polish,
            Language::Portuguese,
            Language::Punjabi,
        ]);
    }

    // --- original 30 languages ---

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
    fn ar_alias_returns_arabic_and_armenian() {
        // "armenian" also starts with "ar"
        assert_eq!(Language::suggestions("ar"), vec![Language::Arabic, Language::Armenian]);
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

    #[test]
    fn es_alias_returns_estonian_and_spanish() {
        // "estonian" also starts with "es"
        assert_eq!(Language::suggestions("es"), vec![Language::Estonian, Language::Spanish]);
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
    fn ta_alias_returns_tagalog_and_tamil() {
        // "tagalog" also starts with "ta"
        assert_eq!(Language::suggestions("ta"), vec![Language::Tagalog, Language::Tamil]);
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
    fn sw_alias_returns_swahili_and_swedish() {
        // "swedish" also starts with "sw"
        assert_eq!(Language::suggestions("sw"), vec![Language::Swahili, Language::Swedish]);
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

    // --- new 45 languages ---

    #[test]
    fn af_alias_returns_afrikaans() {
        assert_eq!(Language::suggestions("af"), vec![Language::Afrikaans]);
    }
    #[test]
    fn afrikaans_alias_returns_afrikaans() {
        assert_eq!(Language::suggestions("afrikaans"), vec![Language::Afrikaans]);
    }
    #[test]
    fn afrikaans_label() { assert_eq!(Language::Afrikaans.label(), "Afrikaans (AF)"); }

    #[test]
    fn sq_alias_returns_albanian() {
        assert_eq!(Language::suggestions("sq"), vec![Language::Albanian]);
    }
    #[test]
    fn albanian_alias_returns_albanian() {
        assert_eq!(Language::suggestions("albanian"), vec![Language::Albanian]);
    }
    #[test]
    fn albanian_label() { assert_eq!(Language::Albanian.label(), "Albanian (SQ)"); }

    #[test]
    fn am_alias_returns_amharic() {
        assert_eq!(Language::suggestions("am"), vec![Language::Amharic]);
    }
    #[test]
    fn amharic_alias_returns_amharic() {
        assert_eq!(Language::suggestions("amharic"), vec![Language::Amharic]);
    }
    #[test]
    fn amharic_script_prefix_returns_amharic() {
        assert_eq!(Language::suggestions("አማ"), vec![Language::Amharic]);
    }
    #[test]
    fn amharic_label() { assert_eq!(Language::Amharic.label(), "Amharic (AM)"); }

    #[test]
    fn hy_alias_returns_armenian() {
        assert_eq!(Language::suggestions("hy"), vec![Language::Armenian]);
    }
    #[test]
    fn armenian_alias_returns_armenian() {
        assert_eq!(Language::suggestions("armenian"), vec![Language::Armenian]);
    }
    #[test]
    fn armenian_script_prefix_returns_armenian() {
        assert_eq!(Language::suggestions("հայ"), vec![Language::Armenian]);
    }
    #[test]
    fn armenian_label() { assert_eq!(Language::Armenian.label(), "Armenian (HY)"); }

    #[test]
    fn az_alias_returns_azerbaijani() {
        assert_eq!(Language::suggestions("az"), vec![Language::Azerbaijani]);
    }
    #[test]
    fn azerbaijani_alias_returns_azerbaijani() {
        assert_eq!(Language::suggestions("azerbaijani"), vec![Language::Azerbaijani]);
    }
    #[test]
    fn azerbaijani_label() { assert_eq!(Language::Azerbaijani.label(), "Azerbaijani (AZ)"); }

    #[test]
    fn eu_alias_returns_basque() {
        assert_eq!(Language::suggestions("eu"), vec![Language::Basque]);
    }
    #[test]
    fn basque_alias_returns_basque() {
        assert_eq!(Language::suggestions("basque"), vec![Language::Basque]);
    }
    #[test]
    fn basque_label() { assert_eq!(Language::Basque.label(), "Basque (EU)"); }

    #[test]
    fn be_alias_returns_belarusian_and_bengali() {
        // "bengali" also starts with "be"
        assert_eq!(Language::suggestions("be"), vec![Language::Belarusian, Language::Bengali]);
    }
    #[test]
    fn belarusian_alias_returns_belarusian() {
        assert_eq!(Language::suggestions("belarusian"), vec![Language::Belarusian]);
    }
    #[test]
    fn belarusian_script_prefix_returns_belarusian() {
        assert_eq!(Language::suggestions("бела"), vec![Language::Belarusian]);
    }
    #[test]
    fn belarusian_label() { assert_eq!(Language::Belarusian.label(), "Belarusian (BE)"); }

    #[test]
    fn bg_alias_returns_bulgarian() {
        assert_eq!(Language::suggestions("bg"), vec![Language::Bulgarian]);
    }
    #[test]
    fn bulgarian_alias_returns_bulgarian() {
        assert_eq!(Language::suggestions("bulgarian"), vec![Language::Bulgarian]);
    }
    #[test]
    fn bulgarian_script_prefix_returns_bulgarian() {
        assert_eq!(Language::suggestions("бъл"), vec![Language::Bulgarian]);
    }
    #[test]
    fn bulgarian_label() { assert_eq!(Language::Bulgarian.label(), "Bulgarian (BG)"); }

    #[test]
    fn my_alias_returns_burmese() {
        assert_eq!(Language::suggestions("my"), vec![Language::Burmese]);
    }
    #[test]
    fn burmese_alias_returns_burmese() {
        assert_eq!(Language::suggestions("burmese"), vec![Language::Burmese]);
    }
    #[test]
    fn burmese_script_prefix_returns_burmese() {
        assert_eq!(Language::suggestions("မြန်"), vec![Language::Burmese]);
    }
    #[test]
    fn burmese_label() { assert_eq!(Language::Burmese.label(), "Burmese (MY)"); }

    #[test]
    fn ca_alias_returns_catalan() {
        assert_eq!(Language::suggestions("ca"), vec![Language::Catalan]);
    }
    #[test]
    fn catalan_alias_returns_catalan() {
        assert_eq!(Language::suggestions("catalan"), vec![Language::Catalan]);
    }
    #[test]
    fn catalan_label() { assert_eq!(Language::Catalan.label(), "Catalan (CA)"); }

    #[test]
    fn hr_alias_returns_croatian() {
        assert_eq!(Language::suggestions("hr"), vec![Language::Croatian]);
    }
    #[test]
    fn croatian_alias_returns_croatian() {
        assert_eq!(Language::suggestions("croatian"), vec![Language::Croatian]);
    }
    #[test]
    fn croatian_label() { assert_eq!(Language::Croatian.label(), "Croatian (HR)"); }

    #[test]
    fn da_alias_returns_danish() {
        assert_eq!(Language::suggestions("da"), vec![Language::Danish]);
    }
    #[test]
    fn danish_alias_returns_danish() {
        assert_eq!(Language::suggestions("danish"), vec![Language::Danish]);
    }
    #[test]
    fn danish_label() { assert_eq!(Language::Danish.label(), "Danish (DA)"); }

    #[test]
    fn et_alias_returns_estonian() {
        assert_eq!(Language::suggestions("et"), vec![Language::Estonian]);
    }
    #[test]
    fn estonian_alias_returns_estonian() {
        assert_eq!(Language::suggestions("estonian"), vec![Language::Estonian]);
    }
    #[test]
    fn estonian_label() { assert_eq!(Language::Estonian.label(), "Estonian (ET)"); }

    #[test]
    fn fi_alias_returns_finnish_and_tagalog() {
        // "filipino" (Tagalog alias) also starts with "fi"
        assert_eq!(Language::suggestions("fi"), vec![Language::Finnish, Language::Tagalog]);
    }
    #[test]
    fn finnish_alias_returns_finnish() {
        assert_eq!(Language::suggestions("finnish"), vec![Language::Finnish]);
    }
    #[test]
    fn finnish_label() { assert_eq!(Language::Finnish.label(), "Finnish (FI)"); }

    #[test]
    fn gl_alias_returns_galician() {
        assert_eq!(Language::suggestions("gl"), vec![Language::Galician]);
    }
    #[test]
    fn galician_alias_returns_galician() {
        assert_eq!(Language::suggestions("galician"), vec![Language::Galician]);
    }
    #[test]
    fn galician_label() { assert_eq!(Language::Galician.label(), "Galician (GL)"); }

    #[test]
    fn ka_alias_returns_georgian_kannada_kazakh() {
        // "kannada" and "kazakh" also start with "ka"
        assert_eq!(Language::suggestions("ka"), vec![Language::Georgian, Language::Kannada, Language::Kazakh]);
    }
    #[test]
    fn georgian_alias_returns_georgian() {
        assert_eq!(Language::suggestions("georgian"), vec![Language::Georgian]);
    }
    #[test]
    fn georgian_script_prefix_returns_georgian() {
        assert_eq!(Language::suggestions("ქართ"), vec![Language::Georgian]);
    }
    #[test]
    fn georgian_label() { assert_eq!(Language::Georgian.label(), "Georgian (KA)"); }

    #[test]
    fn gu_alias_returns_gujarati() {
        assert_eq!(Language::suggestions("gu"), vec![Language::Gujarati]);
    }
    #[test]
    fn gujarati_alias_returns_gujarati() {
        assert_eq!(Language::suggestions("gujarati"), vec![Language::Gujarati]);
    }
    #[test]
    fn gujarati_script_prefix_returns_gujarati() {
        assert_eq!(Language::suggestions("ગુજ"), vec![Language::Gujarati]);
    }
    #[test]
    fn gujarati_label() { assert_eq!(Language::Gujarati.label(), "Gujarati (GU)"); }

    #[test]
    fn ha_alias_returns_hausa() {
        assert_eq!(Language::suggestions("ha"), vec![Language::Hausa]);
    }
    #[test]
    fn hausa_alias_returns_hausa() {
        assert_eq!(Language::suggestions("hausa"), vec![Language::Hausa]);
    }
    #[test]
    fn hausa_label() { assert_eq!(Language::Hausa.label(), "Hausa (HA)"); }

    #[test]
    fn he_alias_returns_hebrew() {
        assert_eq!(Language::suggestions("he"), vec![Language::Hebrew]);
    }
    #[test]
    fn hebrew_alias_returns_hebrew() {
        assert_eq!(Language::suggestions("hebrew"), vec![Language::Hebrew]);
    }
    #[test]
    fn hebrew_script_prefix_returns_hebrew() {
        assert_eq!(Language::suggestions("עבר"), vec![Language::Hebrew]);
    }
    #[test]
    fn hebrew_label() { assert_eq!(Language::Hebrew.label(), "Hebrew (HE)"); }

    #[test]
    fn is_alias_returns_icelandic() {
        assert_eq!(Language::suggestions("is"), vec![Language::Icelandic]);
    }
    #[test]
    fn icelandic_alias_returns_icelandic() {
        assert_eq!(Language::suggestions("icelandic"), vec![Language::Icelandic]);
    }
    #[test]
    fn icelandic_label() { assert_eq!(Language::Icelandic.label(), "Icelandic (IS)"); }

    #[test]
    fn ga_alias_returns_galician_and_irish() {
        // "galician" also starts with "ga"
        assert_eq!(Language::suggestions("ga"), vec![Language::Galician, Language::Irish]);
    }
    #[test]
    fn irish_alias_returns_irish() {
        assert_eq!(Language::suggestions("irish"), vec![Language::Irish]);
    }
    #[test]
    fn irish_label() { assert_eq!(Language::Irish.label(), "Irish (GA)"); }

    #[test]
    fn kn_alias_returns_kannada() {
        assert_eq!(Language::suggestions("kn"), vec![Language::Kannada]);
    }
    #[test]
    fn kannada_alias_returns_kannada() {
        assert_eq!(Language::suggestions("kannada"), vec![Language::Kannada]);
    }
    #[test]
    fn kannada_script_prefix_returns_kannada() {
        assert_eq!(Language::suggestions("ಕನ್"), vec![Language::Kannada]);
    }
    #[test]
    fn kannada_label() { assert_eq!(Language::Kannada.label(), "Kannada (KN)"); }

    #[test]
    fn kk_alias_returns_kazakh() {
        assert_eq!(Language::suggestions("kk"), vec![Language::Kazakh]);
    }
    #[test]
    fn kazakh_alias_returns_kazakh() {
        assert_eq!(Language::suggestions("kazakh"), vec![Language::Kazakh]);
    }
    #[test]
    fn kazakh_label() { assert_eq!(Language::Kazakh.label(), "Kazakh (KK)"); }

    #[test]
    fn km_alias_returns_khmer() {
        assert_eq!(Language::suggestions("km"), vec![Language::Khmer]);
    }
    #[test]
    fn khmer_alias_returns_khmer() {
        assert_eq!(Language::suggestions("khmer"), vec![Language::Khmer]);
    }
    #[test]
    fn khmer_script_prefix_returns_khmer() {
        assert_eq!(Language::suggestions("ខ្មែ"), vec![Language::Khmer]);
    }
    #[test]
    fn khmer_label() { assert_eq!(Language::Khmer.label(), "Khmer (KM)"); }

    #[test]
    fn lv_alias_returns_latvian() {
        assert_eq!(Language::suggestions("lv"), vec![Language::Latvian]);
    }
    #[test]
    fn latvian_alias_returns_latvian() {
        assert_eq!(Language::suggestions("latvian"), vec![Language::Latvian]);
    }
    #[test]
    fn latvian_label() { assert_eq!(Language::Latvian.label(), "Latvian (LV)"); }

    #[test]
    fn lt_alias_returns_lithuanian() {
        assert_eq!(Language::suggestions("lt"), vec![Language::Lithuanian]);
    }
    #[test]
    fn lithuanian_alias_returns_lithuanian() {
        assert_eq!(Language::suggestions("lithuanian"), vec![Language::Lithuanian]);
    }
    #[test]
    fn lithuanian_label() { assert_eq!(Language::Lithuanian.label(), "Lithuanian (LT)"); }

    #[test]
    fn mk_alias_returns_macedonian() {
        assert_eq!(Language::suggestions("mk"), vec![Language::Macedonian]);
    }
    #[test]
    fn macedonian_alias_returns_macedonian() {
        assert_eq!(Language::suggestions("macedonian"), vec![Language::Macedonian]);
    }
    #[test]
    fn macedonian_label() { assert_eq!(Language::Macedonian.label(), "Macedonian (MK)"); }

    #[test]
    fn ms_alias_returns_malay() {
        assert_eq!(Language::suggestions("ms"), vec![Language::Malay]);
    }
    #[test]
    fn malay_alias_returns_malay_and_malayalam() {
        // "malayalam" also starts with "malay"
        assert_eq!(Language::suggestions("malay"), vec![Language::Malay, Language::Malayalam]);
    }
    #[test]
    fn malay_label() { assert_eq!(Language::Malay.label(), "Malay (MS)"); }

    #[test]
    fn ml_alias_returns_malayalam() {
        assert_eq!(Language::suggestions("ml"), vec![Language::Malayalam]);
    }
    #[test]
    fn malayalam_alias_returns_malayalam() {
        assert_eq!(Language::suggestions("malayalam"), vec![Language::Malayalam]);
    }
    #[test]
    fn malayalam_script_prefix_returns_malayalam() {
        assert_eq!(Language::suggestions("മലയ"), vec![Language::Malayalam]);
    }
    #[test]
    fn malayalam_label() { assert_eq!(Language::Malayalam.label(), "Malayalam (ML)"); }

    #[test]
    fn mn_alias_returns_mongolian() {
        assert_eq!(Language::suggestions("mn"), vec![Language::Mongolian]);
    }
    #[test]
    fn mongolian_alias_returns_mongolian() {
        assert_eq!(Language::suggestions("mongolian"), vec![Language::Mongolian]);
    }
    #[test]
    fn mongolian_label() { assert_eq!(Language::Mongolian.label(), "Mongolian (MN)"); }

    #[test]
    fn ne_alias_returns_dutch_and_nepali() {
        // "nederlands" (Dutch alias) also starts with "ne"
        assert_eq!(Language::suggestions("ne"), vec![Language::Dutch, Language::Nepali]);
    }
    #[test]
    fn nepali_alias_returns_nepali() {
        assert_eq!(Language::suggestions("nepali"), vec![Language::Nepali]);
    }
    #[test]
    fn nepali_label() { assert_eq!(Language::Nepali.label(), "Nepali (NE)"); }

    #[test]
    fn no_alias_returns_norwegian() {
        assert_eq!(Language::suggestions("no"), vec![Language::Norwegian]);
    }
    #[test]
    fn norwegian_alias_returns_norwegian() {
        assert_eq!(Language::suggestions("norwegian"), vec![Language::Norwegian]);
    }
    #[test]
    fn norwegian_label() { assert_eq!(Language::Norwegian.label(), "Norwegian (NO)"); }

    #[test]
    fn or_alias_returns_odia() {
        assert_eq!(Language::suggestions("or"), vec![Language::Odia]);
    }
    #[test]
    fn odia_alias_returns_odia() {
        assert_eq!(Language::suggestions("odia"), vec![Language::Odia]);
    }
    #[test]
    fn odia_label() { assert_eq!(Language::Odia.label(), "Odia (OR)"); }

    #[test]
    fn pa_alias_returns_punjabi() {
        assert_eq!(Language::suggestions("pa"), vec![Language::Punjabi]);
    }
    #[test]
    fn punjabi_alias_returns_punjabi() {
        assert_eq!(Language::suggestions("punjabi"), vec![Language::Punjabi]);
    }
    #[test]
    fn punjabi_script_prefix_returns_punjabi() {
        assert_eq!(Language::suggestions("ਪੰਜ"), vec![Language::Punjabi]);
    }
    #[test]
    fn punjabi_label() { assert_eq!(Language::Punjabi.label(), "Punjabi (PA)"); }

    #[test]
    fn sr_alias_returns_serbian() {
        assert_eq!(Language::suggestions("sr"), vec![Language::Serbian]);
    }
    #[test]
    fn serbian_alias_returns_serbian() {
        assert_eq!(Language::suggestions("serbian"), vec![Language::Serbian]);
    }
    #[test]
    fn serbian_label() { assert_eq!(Language::Serbian.label(), "Serbian (SR)"); }

    #[test]
    fn si_alias_returns_sinhala() {
        assert_eq!(Language::suggestions("si"), vec![Language::Sinhala]);
    }
    #[test]
    fn sinhala_alias_returns_sinhala() {
        assert_eq!(Language::suggestions("sinhala"), vec![Language::Sinhala]);
    }
    #[test]
    fn sinhala_script_prefix_returns_sinhala() {
        assert_eq!(Language::suggestions("සිංහ"), vec![Language::Sinhala]);
    }
    #[test]
    fn sinhala_label() { assert_eq!(Language::Sinhala.label(), "Sinhala (SI)"); }

    #[test]
    fn sk_alias_returns_slovak() {
        assert_eq!(Language::suggestions("sk"), vec![Language::Slovak]);
    }
    #[test]
    fn slovak_alias_returns_slovak() {
        assert_eq!(Language::suggestions("slovak"), vec![Language::Slovak]);
    }
    #[test]
    fn slovak_label() { assert_eq!(Language::Slovak.label(), "Slovak (SK)"); }

    #[test]
    fn sl_alias_returns_slovak_and_slovenian() {
        // "slovenčina" (Slovak alias) also starts with "sl"
        assert_eq!(Language::suggestions("sl"), vec![Language::Slovak, Language::Slovenian]);
    }
    #[test]
    fn slovenian_alias_returns_slovenian() {
        assert_eq!(Language::suggestions("slovenian"), vec![Language::Slovenian]);
    }
    #[test]
    fn slovenian_label() { assert_eq!(Language::Slovenian.label(), "Slovenian (SL)"); }

    #[test]
    fn so_alias_returns_somali() {
        assert_eq!(Language::suggestions("so"), vec![Language::Somali]);
    }
    #[test]
    fn somali_alias_returns_somali() {
        assert_eq!(Language::suggestions("somali"), vec![Language::Somali]);
    }
    #[test]
    fn somali_label() { assert_eq!(Language::Somali.label(), "Somali (SO)"); }

    #[test]
    fn sv_alias_returns_swedish() {
        assert_eq!(Language::suggestions("sv"), vec![Language::Swedish]);
    }
    #[test]
    fn swedish_alias_returns_swedish() {
        assert_eq!(Language::suggestions("swedish"), vec![Language::Swedish]);
    }
    #[test]
    fn swedish_label() { assert_eq!(Language::Swedish.label(), "Swedish (SV)"); }

    #[test]
    fn tl_alias_returns_tagalog() {
        assert_eq!(Language::suggestions("tl"), vec![Language::Tagalog]);
    }
    #[test]
    fn tagalog_alias_returns_tagalog() {
        assert_eq!(Language::suggestions("tagalog"), vec![Language::Tagalog]);
    }
    #[test]
    fn tagalog_label() { assert_eq!(Language::Tagalog.label(), "Tagalog (TL)"); }

    #[test]
    fn uz_alias_returns_uzbek() {
        assert_eq!(Language::suggestions("uz"), vec![Language::Uzbek]);
    }
    #[test]
    fn uzbek_alias_returns_uzbek() {
        assert_eq!(Language::suggestions("uzbek"), vec![Language::Uzbek]);
    }
    #[test]
    fn uzbek_label() { assert_eq!(Language::Uzbek.label(), "Uzbek (UZ)"); }

    #[test]
    fn cy_alias_returns_welsh() {
        assert_eq!(Language::suggestions("cy"), vec![Language::Welsh]);
    }
    #[test]
    fn welsh_alias_returns_welsh() {
        assert_eq!(Language::suggestions("welsh"), vec![Language::Welsh]);
    }
    #[test]
    fn welsh_label() { assert_eq!(Language::Welsh.label(), "Welsh (CY)"); }

    #[test]
    fn yo_alias_returns_yoruba() {
        assert_eq!(Language::suggestions("yo"), vec![Language::Yoruba]);
    }
    #[test]
    fn yoruba_alias_returns_yoruba() {
        assert_eq!(Language::suggestions("yoruba"), vec![Language::Yoruba]);
    }
    #[test]
    fn yoruba_label() { assert_eq!(Language::Yoruba.label(), "Yoruba (YO)"); }

    #[test]
    fn zu_alias_returns_zulu() {
        assert_eq!(Language::suggestions("zu"), vec![Language::Zulu]);
    }
    #[test]
    fn zulu_alias_returns_zulu() {
        assert_eq!(Language::suggestions("zulu"), vec![Language::Zulu]);
    }
    #[test]
    fn zulu_label() { assert_eq!(Language::Zulu.label(), "Zulu (ZU)"); }

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
    fn uppercase_es_returns_estonian_and_spanish() {
        assert_eq!(Language::suggestions("ES"), vec![Language::Estonian, Language::Spanish]);
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

    // --- labels (original 30) ---

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
