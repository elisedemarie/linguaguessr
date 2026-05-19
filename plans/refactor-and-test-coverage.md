# Refactor & Test Coverage Plan

Findings from a codebase audit after the componentisation refactor. Split into two tracks: code quality and test gaps.

---

## Track 1 — Refactors

### 1. Derive `Copy` on `Language` and `GameMode`

**Files:** `common/src/types.rs:3,12`

Both enums have no associated data. Add `Copy` to their `derive` macros. This cascades to remove unnecessary `.clone()` calls at:

- `backend/src/handlers.rs:60` — `mode.clone()` in `GameSession` creation
- `backend/src/handlers.rs:109` — `round.language.clone()` in `GuessResponse`
- `common/src/scoring.rs:71` — `key.clone()` in `entry()` lookup
- Various frontend component `.clone()` calls on `Language` values

Highest value single change — touches the most code for the least risk.

---

### 2. Remove `FetchError::TooShort`

**File:** `backend/src/wikipedia.rs:12`

The variant is defined but never constructed. Either:
- Delete it (if short extracts are handled inline and don't need their own error path), or
- Wire it up properly if the intent was to surface it to callers

Currently just dead code with no test coverage.

---

### 3. Remove `LanguageEntry::notes`

**File:** `common/src/scoring.rs:36`

Field is suppressed with `#[allow(dead_code)]`. If it has no planned use, remove it and the allow attribute. If it is planned, document why it exists.

---

### 4. Extract test helpers in `handlers.rs`

**File:** `backend/src/handlers.rs`

Two patterns are repeated ~9 times each in the test module:

- `format!("/api/game/{game_id}/guess")` — extract into `guess_url(game_id: Uuid) -> String`
- `store.lock().unwrap()` insert + retrieve boilerplate — extract into a `seed_session(store, session)` helper or similar

No behaviour changes, just de-duplication.

---

## Track 2 — Missing Test Coverage

### 5. `score_labels()` with identical guess and answer

**File:** `common/src/scoring.rs:166`

All existing tests use different language pairs. Add a test for `score_labels(&lang, &lang)` to pin the "Both X script / Both X family" label output on a perfect match — the exact behaviour surfaced by the score explainer UI.

---

### 6. `truncate_extract()` boundary values

**File:** `backend/src/wikipedia.rs`

The constants `MIN_CHARS` (100) and `MAX_CHARS` (600) have no boundary tests. Add tests for:

- Exactly 100 chars → accepted, no retry
- 99 chars → too short, should trigger retry
- Exactly 600 chars → accepted, no truncation
- 601 chars → truncated to ≤ 600

---

### 7. `truncate_extract()` with no space near the truncation boundary

**File:** `backend/src/wikipedia.rs:134`

The `rfind(' ')` fallback path runs when text exceeds 600 chars but has no space in the truncation zone. This path has no test. Add a case with a long unspaced string to confirm it falls back to the byte boundary without panicking.

---

### 8. Malformed `mode` query parameter

**File:** `backend/src/handlers.rs:38`

`GET /api/game?mode=blah` — no test for what Axum returns when `GameMode` deserialization fails on the query param. Verify the response code and that the server doesn't panic.

---

### 9. Malformed POST body on `/api/game/:id/guess`

**File:** `backend/src/handlers.rs:88`

No test for `POST /api/game/:id/guess` with invalid JSON. Verify the 400/422 response and that the session state is not corrupted by the failed request.
