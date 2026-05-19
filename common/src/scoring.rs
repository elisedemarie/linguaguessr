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
    script_family: String,
    script_branch: String,
    script_chars: Option<String>,
    family: String,
    branch: String,
    sub_branch: String,
    #[allow(dead_code)]
    notes: Option<String>,
}

#[derive(Deserialize)]
struct ScoringConfig {
    script_max: u32,
    family_max: u32,
    unrelated: u32,
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
// Jaccard similarity helpers
// ---------------------------------------------------------------------------

fn jaccard_chars(a: &str, b: &str) -> f64 {
    let set_a: std::collections::HashSet<char> = a.chars().collect();
    let set_b: std::collections::HashSet<char> = b.chars().collect();
    let union = set_a.union(&set_b).count();
    if union == 0 { return 1.0; }
    let intersection = set_a.intersection(&set_b).count();
    intersection as f64 / union as f64
}

fn jaccard_nodes(g: [&str; 3], a: [&str; 3]) -> f64 {
    let set_g: std::collections::HashSet<&str> = g.into_iter().collect();
    let set_a: std::collections::HashSet<&str> = a.into_iter().collect();
    let union = set_g.union(&set_a).count();
    if union == 0 { return 0.0; }
    let intersection = set_g.intersection(&set_a).count();
    intersection as f64 / union as f64
}

// ---------------------------------------------------------------------------
// Scoring functions
// ---------------------------------------------------------------------------

fn compute_script_score(g: &Language, a: &Language) -> u32 {
    if g == a { return 500; }
    let data = get_data();
    let ge = entry(g);
    let ae = entry(a);

    if ge.script == ae.script {
        return match (&ge.script_chars, &ae.script_chars) {
            (Some(gc), Some(ac)) => {
                let j = jaccard_chars(gc, ac);
                (data.config.script_max as f64 * j).round() as u32
            }
            _ => data.config.script_max,
        };
    }

    let j = jaccard_nodes(
        [ge.script_family.as_str(), ge.script_branch.as_str(), ge.script.as_str()],
        [ae.script_family.as_str(), ae.script_branch.as_str(), ae.script.as_str()],
    );
    (data.config.script_max as f64 * j).round() as u32
}

fn compute_family_score(g: &Language, a: &Language) -> u32 {
    if g == a { return 500; }
    let ge = entry(g);
    let ae = entry(a);
    let data = get_data();

    let j = jaccard_nodes(
        [ge.family.as_str(), ge.branch.as_str(), ge.sub_branch.as_str()],
        [ae.family.as_str(), ae.branch.as_str(), ae.sub_branch.as_str()],
    );
    (data.config.family_max as f64 * j).round() as u32
}

pub fn binary_score(correct: bool) -> ScoreBreakdown {
    if correct {
        ScoreBreakdown { script: 500, family: 500, total: 1000 }
    } else {
        ScoreBreakdown { script: 0, family: 0, total: 0 }
    }
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
    } else if ge.script_family == ae.script_family && ge.script_branch == ae.script_branch {
        format!("Both use {} scripts", ge.script_branch)
    } else if ge.script_family == ae.script_family {
        format!("Related {} scripts", ge.script_family)
    } else {
        "Different scripts".to_string()
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


// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Language;

    // --- binary_score ---

    #[test]
    fn binary_score_correct_is_1000() {
        assert_eq!(binary_score(true), ScoreBreakdown { script: 500, family: 500, total: 1000 });
    }

    #[test]
    fn binary_score_wrong_is_0() {
        assert_eq!(binary_score(false), ScoreBreakdown { script: 0, family: 0, total: 0 });
    }

    // --- cross-script Jaccard scoring ---

    #[test]
    fn punjabi_hindi_same_branch_scores_250() {
        let score = partial_score(&Language::Punjabi, &Language::Hindi);
        assert_eq!(score.script, 250);
    }

    #[test]
    fn hindi_tamil_same_family_diff_branch_scores_100() {
        let score = partial_score(&Language::Hindi, &Language::Tamil);
        assert_eq!(score.script, 100);
    }

    #[test]
    fn hindi_thai_same_family_diff_branch_scores_100() {
        let score = partial_score(&Language::Hindi, &Language::Thai);
        assert_eq!(score.script, 100);
    }

    #[test]
    fn thai_khmer_same_branch_scores_250() {
        let score = partial_score(&Language::Thai, &Language::Khmer);
        assert_eq!(score.script, 250);
    }

    #[test]
    fn arabic_hebrew_same_family_diff_branch_scores_100() {
        let score = partial_score(&Language::Arabic, &Language::Hebrew);
        assert_eq!(score.script, 100);
    }

    #[test]
    fn english_russian_unrelated_scripts_scores_0() {
        let score = partial_score(&Language::English, &Language::Russian);
        assert_eq!(score.script, 0);
    }

    #[test]
    fn cross_script_scoring_is_symmetric() {
        assert_eq!(
            partial_score(&Language::Punjabi, &Language::Hindi).script,
            partial_score(&Language::Hindi, &Language::Punjabi).script,
        );
        assert_eq!(
            partial_score(&Language::Arabic, &Language::Hebrew).script,
            partial_score(&Language::Hebrew, &Language::Arabic).script,
        );
    }

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

    // --- Jaccard unit tests ---

    #[test]
    fn jaccard_of_identical_sets_is_1() {
        assert!((jaccard_chars("abc", "abc") - 1.0).abs() < 0.001);
    }

    #[test]
    fn jaccard_of_disjoint_sets_is_0() {
        assert!((jaccard_chars("abc", "xyz") - 0.0).abs() < 0.001);
    }

    #[test]
    fn jaccard_of_partial_overlap_is_correct() {
        // {a,b,c} ∩ {b,c,d} = {b,c}, union = {a,b,c,d} → 2/4 = 0.5
        assert!((jaccard_chars("abc", "bcd") - 0.5).abs() < 0.001);
    }

    #[test]
    fn jaccard_of_both_empty_is_1() {
        assert!((jaccard_chars("", "") - 1.0).abs() < 0.001);
    }

    #[test]
    fn jaccard_nodes_same_path_is_1() {
        let j = jaccard_nodes(["IE", "Romance", "Iberian"], ["IE", "Romance", "Iberian"]);
        assert!((j - 1.0).abs() < 0.001);
    }

    #[test]
    fn jaccard_nodes_deduplication_works() {
        // When branch == sub_branch, the set has 2 elements not 3
        let j = jaccard_nodes(["Japonic", "Japonic", "Japonic"], ["Koreanic", "Koreanic", "Koreanic"]);
        // {Japonic} ∩ {Koreanic} = 0, union = 2 → 0
        assert!((j - 0.0).abs() < 0.001);
    }

    // --- representative pair scores (Jaccard-based) ---

    #[test]
    fn spanish_portuguese_scores_838() {
        // Both Latin (script Jaccard ~0.775 → 388), same Iberian Romance (family 1.0 → 450)
        assert_eq!(partial_score(&Language::Spanish, &Language::Portuguese),
            ScoreBreakdown { script: 388, family: 450, total: 838 });
    }

    #[test]
    fn russian_bulgarian_scores_680() {
        // Cyrillic Jaccard 30/33 ≈ 0.909 → 455; East vs South Slavic 2/4 → 225
        assert_eq!(partial_score(&Language::Russian, &Language::Bulgarian),
            ScoreBreakdown { script: 455, family: 225, total: 680 });
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
        assert_eq!(partial_score(&Language::Hindi, &Language::Urdu),
            ScoreBreakdown { script: 0, family: 450, total: 450 });
    }

    #[test]
    fn english_japanese_scores_0() {
        assert_eq!(partial_score(&Language::English, &Language::Japanese),
            ScoreBreakdown { script: 0, family: 0, total: 0 });
    }

    #[test]
    fn turkish_azerbaijani_scores_935() {
        // Latin Jaccard 32/33 ≈ 0.970 → 485; both Oghuz → 450
        assert_eq!(partial_score(&Language::Turkish, &Language::Azerbaijani),
            ScoreBreakdown { script: 485, family: 450, total: 935 });
    }

    #[test]
    fn finnish_hungarian_scores_600() {
        // Latin Jaccard 27/36 = 0.75 → 375; Finnic vs Ugric 2/4 → 225
        assert_eq!(partial_score(&Language::Finnish, &Language::Hungarian),
            ScoreBreakdown { script: 375, family: 225, total: 600 });
    }

    #[test]
    fn arabic_hebrew_scores_325() {
        // Semitic cross-script (Arabic Abjad vs Hebrew Abjad) → 100; Semitic branch 2/4 → 225
        assert_eq!(partial_score(&Language::Arabic, &Language::Hebrew),
            ScoreBreakdown { script: 100, family: 225, total: 325 });
    }

    #[test]
    fn english_french_scores_407() {
        // Latin Jaccard 26/41 ≈ 0.634 → 317; IE-only family 1/5 → 90
        assert_eq!(partial_score(&Language::English, &Language::French),
            ScoreBreakdown { script: 317, family: 90, total: 407 });
    }

    #[test]
    fn russian_ukrainian_scores_842() {
        // Cyrillic Jaccard 29/37 ≈ 0.784 → 392; both East Slavic → 450
        assert_eq!(partial_score(&Language::Russian, &Language::Ukrainian),
            ScoreBreakdown { script: 392, family: 450, total: 842 });
    }

    #[test]
    fn basque_spanish_scores_409() {
        // Latin Jaccard 27/33 ≈ 0.818 → 409; Basque isolate vs IE → 0
        assert_eq!(partial_score(&Language::Basque, &Language::Spanish),
            ScoreBreakdown { script: 409, family: 0, total: 409 });
    }

    #[test]
    fn tamil_telugu_scores_700() {
        // Both Brahmic/South Indic → script 250; both South Dravidian → family 450
        assert_eq!(partial_score(&Language::Tamil, &Language::Telugu),
            ScoreBreakdown { script: 250, family: 450, total: 700 });
    }

    #[test]
    fn swahili_zulu_scores_950() {
        // Both base Latin (Jaccard 1.0 → 500), sibling Bantu → 450
        assert_eq!(partial_score(&Language::Swahili, &Language::Zulu),
            ScoreBreakdown { script: 500, family: 450, total: 950 });
    }

    #[test]
    fn japanese_korean_scores_0() {
        assert_eq!(partial_score(&Language::Japanese, &Language::Korean),
            ScoreBreakdown { script: 0, family: 0, total: 0 });
    }

    #[test]
    fn indonesian_malay_scores_950() {
        // Both base Latin (Jaccard 1.0 → 500), sibling Malayo-Polynesian → 450
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
    fn labels_for_same_family_same_branch_different_script() {
        // Punjabi (Gurmukhi) and Hindi (Devanagari): both Brahmic / North Indic
        let labels = score_labels(&Language::Punjabi, &Language::Hindi);
        assert_eq!(labels.script, "Both use North Indic scripts");
    }

    #[test]
    fn labels_for_same_family_different_branch() {
        // Hindi (North Indic) and Tamil (South Indic): both Brahmic
        let labels = score_labels(&Language::Hindi, &Language::Tamil);
        assert_eq!(labels.script, "Related Brahmic scripts");
    }

    #[test]
    fn labels_for_cjk_pair() {
        // Chinese and Japanese: both CJK / Logographic
        let labels = score_labels(&Language::Chinese, &Language::Japanese);
        assert_eq!(labels.script, "Both use Logographic scripts");
    }

    #[test]
    fn labels_for_different_scripts() {
        let labels = score_labels(&Language::English, &Language::Arabic);
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
