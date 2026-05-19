# Plan: Hierarchical Script Family Scoring

**Status:** Ready to implement
**Branch:** `feat/script-family-scoring` (to be created)

---

## Problem

The script scoring axis currently treats all different-script pairs as 0 unless explicitly
listed in `script_special_cases`. This produces jarring results:

- Punjabi (Gurmukhi) vs Hindi (Devanagari) → 0 script score, despite being visually and
  historically close siblings within the Brahmic family
- Tamil vs Telugu → 0, despite both being South Indic Brahmic scripts
- Chinese vs Japanese → 250 only because of a manually maintained special case

The special cases mechanism is an ad hoc patch on top of a binary same/different model.
It cannot express degrees of relatedness (North Indic is closer to North Indic than to
Southeast Asian) and requires a human to enumerate every pair explicitly.

---

## Solution

Mirror the language family scoring. Give scripts a 3-node hierarchy:

```
script_family → script_branch → script
```

For **same-script** pairs: keep the existing Jaccard-on-characters scoring (unchanged).
For **different-script** pairs: compute `jaccard_nodes([family, branch, script])` × `script_max`,
exactly as the language family axis does.

The `script_special_cases` block is retired entirely — CJK falls out naturally because
Chinese and Japanese share `script_family = "CJK"`.

---

## Score examples (script_max = 500)

| Pair | Relationship | Jaccard | Script score |
|------|-------------|---------|--------------|
| Hindi / Nepali | Same script (Devanagari), ~95% char overlap | ~0.95 | ~475 |
| Hindi / Punjabi | Same family+branch (N. Indic), diff script | 2/4 = 0.50 | 250 |
| Hindi / Tamil | Same family (Brahmic), diff branch | 1/5 = 0.20 | 100 |
| Hindi / Thai | Same family (Brahmic), diff branch | 1/5 = 0.20 | 100 |
| Chinese / Japanese | Same family+branch (CJK/Sinitic→replaces special case) | 2/4 = 0.50 | 250 |
| Arabic / Hebrew | Same family (Semitic), diff branch | 1/5 = 0.20 | 100 |
| English / Russian | Unrelated families | 0/n = 0 | 0 |

---

## Script hierarchy

```
Brahmic
  ├── North Indic   → Devanagari (Hindi, Marathi, Nepali), Gurmukhi (Punjabi),
  │                   Bengali, Gujarati, Odia, Sinhala
  ├── South Indic   → Tamil, Telugu, Kannada, Malayalam
  └── Southeast Asian → Thai, Khmer, Myanmar (Burmese)

Semitic
  ├── Arabic        → Arabic, Persian, Urdu  ← already same script, unchanged
  └── Hebrew        → Hebrew
  └── Ethiopic      → Amharic

CJK
  ├── Sinitic       → Chinese
  └── Japanese      → Japanese

Latin             → all Latin-script languages  ← already same script, unchanged
Cyrillic          → all Cyrillic-script languages ← already same script, unchanged

Unique (no relatives worth scoring — isolated within their own family/branch/script)
  → Greek, Georgian, Armenian, Korean
```

Note: Latin and Cyrillic share a distant Greek ancestor but are kept as separate families.
They look nothing alike in practice and rewarding "guessed Latin when it was Cyrillic" would
feel wrong to players.

---

## TOML changes

### 1. Update the field documentation header

Add two new fields to the per-language documentation block at the top of `languages.toml`:

```toml
#   script_family — the broad script lineage for cross-script partial scoring
#                  (e.g. "Brahmic", "Semitic", "CJK", "Latin", "Cyrillic")
#                  languages with no close script relatives use their script name
#   script_branch — the sub-group within the script family
#                  (e.g. "North Indic", "South Indic", "Southeast Asian")
#                  used together with script_family for Jaccard node scoring
```

### 2. Add a script hierarchy reference block

Add a human-readable reference just below `[scoring]`, before the language entries.
This is the single source of truth for what groupings exist and why — the place a
linguist would go to suggest a change:

```toml
# ---------------------------------------------------------------------------
# Script hierarchy — used for cross-script partial scoring
#
# When a guess uses a different script from the answer, the score is computed
# as Jaccard([script_family, script_branch, script]) × script_max.
#
# To suggest a change to a grouping, open a GitHub issue referencing this block.
#
# Defined families:
#
#   Brahmic         Scripts descended from the ancient Brahmi alphabet
#     North Indic   Devanagari · Gurmukhi · Bengali · Gujarati · Odia · Sinhala
#     South Indic   Tamil · Telugu · Kannada · Malayalam
#     SE Asian      Thai · Khmer · Myanmar
#
#   Semitic         Consonantal scripts from the Semitic writing tradition
#     Arabic        Arabic · Persian · Urdu  (already share script= field)
#     Hebrew        Hebrew
#     Ethiopic      Amharic (Ge'ez-derived)
#
#   CJK             Logographic East Asian scripts
#     Sinitic       Chinese
#     Japanese      Japanese
#
#   Latin           All Latin-alphabet languages  (already share script= field)
#   Cyrillic        All Cyrillic-alphabet languages  (already share script= field)
#
#   Scripts with no close relatives use their own name as both family and branch:
#   Greek · Georgian · Armenian · Korean
# ---------------------------------------------------------------------------
```

### 3. Remove `[[scoring.script_special_cases]]`

The entire special-cases block is deleted. Its only entry (Chinese/Japanese) is now
handled by the hierarchy.

### 4. Per-language entries — add two fields, example:

```toml
[languages.Hindi]
script        = "Devanagari"
script_family = "Brahmic"
script_branch = "North Indic"
script_chars  = "..."
family        = "Indo-European"
branch        = "Indo-Iranian"
sub_branch    = "Indo-Aryan"

[languages.Punjabi]
script        = "Gurmukhi"
script_family = "Brahmic"
script_branch = "North Indic"
# no script_chars — single-language script, Jaccard not applicable
family        = "Indo-European"
branch        = "Indo-Iranian"
sub_branch    = "Indo-Aryan"

[languages.Greek]
script        = "Greek"
script_family = "Greek"    # no close relatives — own family
script_branch = "Greek"
family        = "Indo-European"
branch        = "Hellenic"
sub_branch    = "Hellenic"
```

Field order convention: `script` · `script_family` · `script_branch` · `script_chars` · then language fields.
This keeps all script-related fields together and makes the grouping immediately visible
when reading or auditing a language entry.

---

## Code changes (`common/src/scoring.rs`)

### 1. Update `LanguageEntry` to deserialise the new fields

```rust
struct LanguageEntry {
    script: String,
    script_family: String,
    script_branch: String,
    script_chars: Option<String>,
    ...
}
```

### 2. Update `compute_script_score`

```rust
fn compute_script_score(g: &Language, a: &Language) -> u32 {
    if g == a { return 500; }
    let ge = entry(g);
    let ae = entry(a);

    // Same script → Jaccard on characters (existing behaviour, unchanged)
    if ge.script == ae.script {
        return match (&ge.script_chars, &ae.script_chars) {
            (Some(gc), Some(ac)) => {
                let j = jaccard_chars(gc, ac);
                (data.config.script_max as f64 * j).round() as u32
            }
            _ => data.config.script_max,
        };
    }

    // Different script → Jaccard on script hierarchy nodes
    let j = jaccard_nodes(
        [ge.script_family.as_str(), ge.script_branch.as_str(), ge.script.as_str()],
        [ae.script_family.as_str(), ae.script_branch.as_str(), ae.script.as_str()],
    );
    (data.config.script_max as f64 * j).round() as u32
}
```

### 3. Remove special-cases infrastructure

- Delete `ScriptSpecialCase` struct
- Delete `script_special_cases` field from `ScoringConfig`
- Delete the special-cases loop from `compute_script_score`
- Delete the special-cases branch from `score_labels`

### 4. Update `score_labels` for the new cross-script cases

Currently returns "Different scripts" for all unrelated pairs and "Both use CJK characters"
for the Chinese/Japanese special case. New logic:

```
same script          → "Both {script} script"           (unchanged)
same family+branch   → "Both use {branch} scripts"      (e.g. "Both use North Indic scripts")
same family only     → "Related {family} scripts"       (e.g. "Related Brahmic scripts")
unrelated            → "Different scripts"               (unchanged)
```

---

## Implementation slices (TDD order)

### Slice 1 — TOML data (no code, just data)
Add `script_family` and `script_branch` to all 75 language entries. The existing
`all_languages_have_toml_entries` test will catch any missing or misspelled field
once the struct is updated in Slice 2.

### Slice 2 — Deserialise new fields
- RED: update `LanguageEntry` struct → existing tests fail to compile
- GREEN: add fields, confirm all tests pass (TOML data already has the fields from Slice 1)
- MUTATE

### Slice 3 — Cross-script Jaccard scoring
- RED: write representative pair tests (Hindi/Punjabi → 250, Hindi/Tamil → 100, English/Russian → 0)
- GREEN: update `compute_script_score` to use `jaccard_nodes` for different-script pairs
- MUTATE

### Slice 4 — Remove special cases
- RED: confirm the CJK pair (Chinese/Japanese) still scores 250 via the new path
- GREEN: delete `ScriptSpecialCase`, `script_special_cases`, and the special-cases loop
- MUTATE

### Slice 5 — Update score_labels
- RED: write label tests for same-family, same-branch, and different-family cases
- GREEN: update `score_labels` to use the three-tier label logic
- MUTATE

---

## Out of scope

- Changing `script_max` (still 500)
- Adding more script families beyond those listed (future work)
- Mobile tooltip fix (tracked separately in `plans/mvp.md`)
