use std::collections::HashMap;
use std::sync::OnceLock;
use serde::Deserialize;
use crate::types::Language;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ScoreBreakdown {
    pub script: u32,
    pub family: u32,
    pub total: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ScoreLabels {
    pub script: String,
    pub family: String,
}

// ---------------------------------------------------------------------------
// TOML data model (private — parsing internals)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct LanguageEntry {
    script: String,
    family: String,
    branch: String,
    sub_branch: String,
    #[allow(dead_code)]
    notes: Option<String>,
}

#[derive(Deserialize)]
struct ScriptSpecialCase {
    languages: Vec<String>,
    score: u32,
    #[allow(dead_code)]
    reason: String,
}

#[derive(Deserialize)]
struct ScoringConfig {
    same_script: u32,
    same_sub_branch: u32,
    same_branch: u32,
    same_family: u32,
    unrelated: u32,
    script_special_cases: Vec<ScriptSpecialCase>,
}

#[derive(Deserialize)]
struct RawData {
    scoring: ScoringConfig,
    languages: HashMap<String, LanguageEntry>,
}

// ---------------------------------------------------------------------------
// Parsed + resolved data (keyed by Language for fast lookup)
// ---------------------------------------------------------------------------

struct ScoringData {
    config: ScoringConfig,
    entries: HashMap<Language, LanguageEntry>,
}

static DATA: OnceLock<ScoringData> = OnceLock::new();

fn get_data() -> &'static ScoringData {
    DATA.get_or_init(|| {
        let raw_str = include_str!("../../data/languages.toml");
        let raw: RawData = toml::from_str(raw_str)
            .expect("data/languages.toml is invalid — check TOML syntax");

        let entries = raw.languages.into_iter().map(|(key, entry)| {
            let lang: Language = serde_json::from_value(serde_json::Value::String(key.clone()))
                .unwrap_or_else(|_| panic!("Unknown language name in data/languages.toml: '{key}'"));
            (lang, entry)
        }).collect();

        ScoringData { config: raw.scoring, entries }
    })
}

fn entry(lang: &Language) -> &'static LanguageEntry {
    let data = get_data();
    data.entries.get(lang)
        .unwrap_or_else(|| {
            panic!(
                "Language '{lang:?}' has no entry in data/languages.toml — add it to the file"
            )
        })
}

// ---------------------------------------------------------------------------
// Scoring functions
// ---------------------------------------------------------------------------

fn compute_script_score(g: &Language, a: &Language) -> u32 {
    if g == a { return 500; }
    let data = get_data();
    let ge = entry(g);
    let ae = entry(a);
    if ge.script == ae.script { return data.config.same_script; }
    for special in &data.config.script_special_cases {
        let names = &special.languages;
        let g_name = lang_name(g);
        let a_name = lang_name(a);
        if (names.contains(&g_name) && names.contains(&a_name)) && g_name != a_name {
            return special.score;
        }
    }
    data.config.unrelated
}

fn compute_family_score(g: &Language, a: &Language) -> u32 {
    if g == a { return 500; }
    let data = get_data();
    let ge = entry(g);
    let ae = entry(a);
    if ge.sub_branch == ae.sub_branch { return data.config.same_sub_branch; }
    if ge.branch == ae.branch { return data.config.same_branch; }
    if ge.family == ae.family { return data.config.same_family; }
    data.config.unrelated
}

pub fn partial_score(guess: &Language, answer: &Language) -> ScoreBreakdown {
    let script = compute_script_score(guess, answer);
    let family = compute_family_score(guess, answer);
    ScoreBreakdown { script, family, total: script + family }
}

pub fn score_labels(guess: &Language, answer: &Language) -> ScoreLabels {
    let ge = entry(guess);
    let ae = entry(answer);

    let script = if ge.script == ae.script {
        format!("Both {} script", ge.script)
    } else {
        let data = get_data();
        let g_name = lang_name(guess);
        let a_name = lang_name(answer);
        let is_special = data.config.script_special_cases.iter().any(|sc| {
            sc.languages.contains(&g_name) && sc.languages.contains(&a_name) && g_name != a_name
        });
        if is_special {
            "Both use CJK characters".to_string()
        } else {
            "Different scripts".to_string()
        }
    };

    let family = if ge.sub_branch == ae.sub_branch {
        format!("Both {}", ge.sub_branch)
    } else if ge.branch == ae.branch {
        format!("Both {} languages", ge.branch)
    } else if ge.family == ae.family {
        format!("Both {} family", ge.family)
    } else {
        "Different language families".to_string()
    };

    ScoreLabels { script, family }
}

fn lang_name(lang: &Language) -> String {
    serde_json::to_value(lang)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Language;

    // --- completeness: every Language variant must be in the TOML ---

    #[test]
    fn all_languages_have_toml_entries() {
        for lang in Language::all() {
            let _ = entry(lang); // panics with language name if missing
        }
    }

    // --- correct answer always scores 1000 ---

    #[test]
    fn correct_answer_scores_1000() {
        for lang in Language::all() {
            let score = partial_score(lang, lang);
            assert_eq!(score, ScoreBreakdown { script: 500, family: 500, total: 1000 },
                "Language {lang:?} did not score 1000 against itself");
        }
    }

    // --- representative pair scores ---

    #[test]
    fn spanish_portuguese_scores_950() {
        // Both Latin script, sibling Iberian Romance
        assert_eq!(partial_score(&Language::Spanish, &Language::Portuguese),
            ScoreBreakdown { script: 500, family: 450, total: 950 });
    }

    #[test]
    fn russian_bulgarian_scores_800() {
        // Both Cyrillic, same Slavic family but East vs South sub-branch
        assert_eq!(partial_score(&Language::Russian, &Language::Bulgarian),
            ScoreBreakdown { script: 500, family: 300, total: 800 });
    }

    #[test]
    fn chinese_japanese_scores_250() {
        // CJK special case (shared kanji), unrelated families
        assert_eq!(partial_score(&Language::Chinese, &Language::Japanese),
            ScoreBreakdown { script: 250, family: 0, total: 250 });
    }

    #[test]
    fn hindi_urdu_scores_450() {
        // Different scripts (Devanagari vs Arabic), sibling Indo-Aryan
        // Self-determination: scored strictly on attributes, no special-casing
        assert_eq!(partial_score(&Language::Hindi, &Language::Urdu),
            ScoreBreakdown { script: 0, family: 450, total: 450 });
    }

    #[test]
    fn english_japanese_scores_0() {
        // Latin vs Japanese script, Indo-European vs Japonic
        assert_eq!(partial_score(&Language::English, &Language::Japanese),
            ScoreBreakdown { script: 0, family: 0, total: 0 });
    }

    #[test]
    fn turkish_azerbaijani_scores_950() {
        // Both Latin script, sibling Oghuz Turkic
        assert_eq!(partial_score(&Language::Turkish, &Language::Azerbaijani),
            ScoreBreakdown { script: 500, family: 450, total: 950 });
    }

    #[test]
    fn finnish_hungarian_scores_800() {
        // Both Latin script, same Uralic family but Finnic vs Ugric sub-branch
        assert_eq!(partial_score(&Language::Finnish, &Language::Hungarian),
            ScoreBreakdown { script: 500, family: 300, total: 800 });
    }

    #[test]
    fn arabic_hebrew_scores_300() {
        // Different Semitic scripts, same Semitic branch
        assert_eq!(partial_score(&Language::Arabic, &Language::Hebrew),
            ScoreBreakdown { script: 0, family: 300, total: 300 });
    }

    #[test]
    fn english_french_scores_650() {
        // Both Latin, same Indo-European family but Germanic vs Romance branch
        assert_eq!(partial_score(&Language::English, &Language::French),
            ScoreBreakdown { script: 500, family: 150, total: 650 });
    }

    #[test]
    fn russian_ukrainian_scores_950() {
        // Both Cyrillic, sibling East Slavic
        assert_eq!(partial_score(&Language::Russian, &Language::Ukrainian),
            ScoreBreakdown { script: 500, family: 450, total: 950 });
    }

    #[test]
    fn tamil_telugu_scores_800() {
        // Tamil has Tamil script, Telugu has Telugu script — different scripts
        // Both South Dravidian sub-branch
        assert_eq!(partial_score(&Language::Tamil, &Language::Telugu),
            ScoreBreakdown { script: 0, family: 450, total: 450 });
    }

    #[test]
    fn swahili_zulu_scores_950() {
        // Both Latin script, sibling Bantu
        assert_eq!(partial_score(&Language::Swahili, &Language::Zulu),
            ScoreBreakdown { script: 500, family: 450, total: 950 });
    }

    #[test]
    fn japanese_korean_scores_0() {
        // Different scripts, different language families (Japonic vs Koreanic)
        assert_eq!(partial_score(&Language::Japanese, &Language::Korean),
            ScoreBreakdown { script: 0, family: 0, total: 0 });
    }

    #[test]
    fn basque_spanish_scores_500() {
        // Both Latin script, Basque is an isolate — no family relation
        assert_eq!(partial_score(&Language::Basque, &Language::Spanish),
            ScoreBreakdown { script: 500, family: 0, total: 500 });
    }

    #[test]
    fn indonesian_malay_scores_950() {
        // Both Latin, sibling Malayo-Polynesian
        assert_eq!(partial_score(&Language::Indonesian, &Language::Malay),
            ScoreBreakdown { script: 500, family: 450, total: 950 });
    }

    // --- symmetry: score(A, B) == score(B, A) ---

    #[test]
    fn scoring_is_symmetric() {
        let pairs = [
            (Language::Spanish, Language::French),
            (Language::Chinese, Language::Japanese),
            (Language::Hindi, Language::Urdu),
            (Language::Finnish, Language::Hungarian),
        ];
        for (a, b) in &pairs {
            assert_eq!(partial_score(a, b), partial_score(b, a),
                "Score not symmetric for {a:?} / {b:?}");
        }
    }

    // --- score_labels ---

    #[test]
    fn labels_for_same_script_match() {
        let labels = score_labels(&Language::Spanish, &Language::French);
        assert_eq!(labels.script, "Both Latin script");
    }

    #[test]
    fn labels_for_cjk_special_case() {
        let labels = score_labels(&Language::Chinese, &Language::Japanese);
        assert_eq!(labels.script, "Both use CJK characters");
    }

    #[test]
    fn labels_for_different_scripts() {
        let labels = score_labels(&Language::English, &Language::Arabic);
        assert_eq!(labels.script, "Different scripts");
    }

    #[test]
    fn cjk_label_requires_both_languages_in_pair_not_just_one() {
        // Chinese is in the CJK special case, but Spanish is not —
        // only one side of the pair matches, so label must not be CJK
        let labels = score_labels(&Language::Chinese, &Language::Spanish);
        assert_eq!(labels.script, "Different scripts");
    }

    #[test]
    fn labels_for_sub_branch_match() {
        let labels = score_labels(&Language::Spanish, &Language::Portuguese);
        assert_eq!(labels.family, "Both Iberian Romance");
    }

    #[test]
    fn labels_for_branch_match() {
        let labels = score_labels(&Language::Spanish, &Language::French);
        assert_eq!(labels.family, "Both Romance languages");
    }

    #[test]
    fn labels_for_family_match() {
        let labels = score_labels(&Language::English, &Language::French);
        assert_eq!(labels.family, "Both Indo-European family");
    }

    #[test]
    fn labels_for_no_match() {
        let labels = score_labels(&Language::English, &Language::Japanese);
        assert_eq!(labels.family, "Different language families");
    }
}
