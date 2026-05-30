# Plan: Daily Challenge

**Branch**: feat/daily-challenge
**Status**: Active

## Goal

Every day, all players get the same 5 languages to identify — with a shareable emoji result they can copy after finishing.

## Acceptance Criteria

- [ ] `GET /api/game?mode=daily` returns the same 5 languages for all callers on the same UTC day
- [ ] Two calls to `GET /api/game?mode=daily` on the same day return different `game_id`s but the same ordered set of 5 languages
- [ ] Two calls on different UTC dates return different language sets
- [ ] The languages are drawn from the full 75-language pool (`Language::all()`)
- [ ] Daily mode uses partial scoring (same as Medium/Hard — not binary like Easy)
- [ ] The home screen shows a "Daily Challenge" button distinct from Easy/Medium/Hard
- [ ] After completing a daily game, the score screen shows a "Daily Challenge Complete" heading and a copyable emoji result grid
- [ ] If the player has already completed today's daily (stored in `localStorage`), the home screen shows their result instead of the button

## Slices

Every slice follows RED-GREEN-MUTATE-KILL MUTANTS-REFACTOR. No production code without a failing test.
Load `tdd`, `testing`, `mutation-testing`, and `refactoring` before code changes begin on each slice.

---

### Slice 1: `Language::daily_languages(date)` — deterministic language selection in `common`

**Value**: The core daily mechanic — a pure function that maps a date to 5 languages, fully testable without any I/O.

**Path**: Pure function in `common/src/types.rs`. Takes a `chrono::NaiveDate`, seeds a deterministic RNG (e.g. `rand_chacha::ChaCha8Rng`) from `date.num_days_from_ce() as u64`, shuffles `Language::all()`, returns the first 5 as `[Language; 5]`. No backend or frontend changes in this slice.

**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, `refactoring` before writing code.

**Acceptance criteria**:
- `daily_languages(date_a) == daily_languages(date_a)` — same date, same result
- `daily_languages(date_a) != daily_languages(date_b)` for dates one day apart
- Result always contains exactly 5 distinct languages
- All 5 are from `Language::all()`
- Present to human and get confirmation before writing any code.

**RED**: Tests in `common/src/types.rs` — fixed `NaiveDate` values, assert determinism, distinctness, count, pool membership.

**GREEN**: Add `chrono` and `rand_chacha` to `common/Cargo.toml`. Implement `daily_languages(date: chrono::NaiveDate) -> [Language; 5]`.

**MUTATE**: Run `mutation-testing` skill — produce a report.

**KILL MUTANTS**: Address surviving mutants (ask human when value is ambiguous).

**REFACTOR**: Assess improvements (only if they add value).

**Done when**: All acceptance criteria met, mutation report reviewed, human approves commit.

---

### Slice 2: `GET /api/game?mode=daily` returns today's 5 languages

**Value**: The backend endpoint that makes daily mode real — callable from the frontend and by anyone who wants to build on top of it.

**Path**: Add `GameMode::Daily` variant to `common/src/types.rs` (serde: `"daily"`). In `backend/src/handlers.rs`, branch the `get_game` handler: if `mode == Daily`, call `Language::daily_languages(Utc::now().date_naive())` instead of the existing shuffle. Daily uses partial scoring (same branch as Medium/Hard — not `binary_score`). `language_pool()` does not need to handle Daily; the branch is directly in the handler.

**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, `refactoring` before writing code.

**Acceptance criteria**:
- `GET /api/game?mode=daily` returns 200 with 5 rounds
- Two requests on the same day return the same 5 languages (verifiable in tests by injecting a fixed date)
- Session is stored with `mode: Daily`
- Partial scoring applies (not binary): a near-miss guess on a Daily game scores > 0
- `GET /api/game?mode=daily` with a malformed value still returns 400 (existing behaviour unchanged)
- Present to human and get confirmation before writing any code.

**RED**: Extend `handlers.rs` tests — inject a fixed date into `get_game` to assert determinism. Test that two sessions created with the same date share the same language set. Test partial scoring applies.

**GREEN**: Add `GameMode::Daily` to enum + serde. Extract a `select_languages(mode, date)` helper in the handler (or inline the branch). Wire `chrono::Utc::now().date_naive()` in the real handler path.

**MUTATE**: Run `mutation-testing` skill — produce a report.

**KILL MUTANTS**: Address surviving mutants.

**REFACTOR**: Assess improvements.

**Done when**: All acceptance criteria met, mutation report reviewed, human approves commit.

---

### Slice 3: Home screen "Daily Challenge" button

**Value**: Players can discover and start a daily game from the home screen.

**Path**: Frontend `frontend/src/components/home_screen.rs`. Add a "Daily Challenge" button styled distinctly from the Easy/Medium/Hard row. Clicking it calls `GET /api/game?mode=daily` and transitions to the round screen, exactly as the other modes do.

**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, `refactoring` before writing code. For pure Leptos rendering with no extractable pure logic, explain and ask permission before skipping RED.

**Acceptance criteria**:
- A "Daily Challenge" button is visible on the home screen
- Clicking it starts a game via `?mode=daily`
- The game plays identically to Hard mode (no options combobox, partial scoring)
- The other mode buttons are unaffected
- Present to human and get confirmation before writing any code.

**RED**: If any pure logic is introduced (e.g. a function that decides which mode to send), write a failing test for it first. For pure rendering changes, explain and ask permission to skip RED.

**GREEN**: Add the button to `home_screen.rs`.

**MUTATE / KILL MUTANTS / REFACTOR**: As above, scoped to any extractable logic.

**Done when**: All acceptance criteria met, human approves commit.

---

### Slice 4: Score screen shows daily completion state and shareable emoji grid

**Value**: Players get a satisfying end-of-daily screen and can share their result (Wordle-style).

**Path**: Frontend `frontend/src/components/finished_screen.rs` and `frontend/src/score.rs` (or a new `daily_share.rs`). When `game_mode == Daily`, show "Daily Challenge Complete – YYYY-MM-DD" heading. Below the score, show an emoji grid: 🟩 for correct (score = 1000), 🟨 for partial (0 < score < 1000), 🟥 for wrong (score = 0). A "Copy result" button copies the grid + score + `linguaguessr.io` to clipboard.

**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, `refactoring` before writing code.

**Acceptance criteria**:
- `round_result_emoji(score)` returns 🟩 / 🟨 / 🟥 correctly
- `format_share_text(date, emojis, total_score)` returns the expected share string
- Score screen shows the daily heading and emoji grid when mode is Daily
- "Copy result" button copies the share text to clipboard
- Non-daily games are unaffected
- Present to human and get confirmation before writing any code.

**RED**: `round_result_emoji` and `format_share_text` are pure functions — write failing tests first.

**GREEN**: Implement both functions, wire into finished screen conditionally.

**MUTATE**: Run `mutation-testing` skill — produce a report.

**KILL MUTANTS**: Address surviving mutants.

**REFACTOR**: Assess improvements.

**Done when**: All acceptance criteria met, mutation report reviewed, human approves commit.

---

### Slice 5: `localStorage` "already played today" guard

**Value**: Players see their result if they've already completed today's daily, rather than being able to replay it silently.

**Path**: Frontend. After a daily game completes, write `{ date: "YYYY-MM-DD", emoji: "🟩🟨🟥🟩🟩", score: 3500 }` to `localStorage["linguaguessr_daily"]`. On home screen mount, read `localStorage["linguaguessr_daily"]`; if the date matches today's UTC date, show the stored result summary instead of the "Daily Challenge" button. The backend does not enforce this — it is a UX-only guard.

**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, `refactoring` before writing code.

**Acceptance criteria**:
- `daily_already_played(stored_entry, today)` returns `true` only when `stored_entry.date == today`
- `daily_already_played` returns `false` when stored date is yesterday or missing
- Home screen shows stored result summary when already played today
- Home screen shows the Daily button when not yet played or date has rolled over
- Present to human and get confirmation before writing any code.

**RED**: `daily_already_played` is a pure function — failing test first.

**GREEN**: Implement the function, wire localStorage read/write at game completion and home screen mount.

**MUTATE**: Run `mutation-testing` skill — produce a report.

**KILL MUTANTS**: Address surviving mutants.

**REFACTOR**: Assess improvements.

**Done when**: All acceptance criteria met, mutation report reviewed, human approves commit.

---

## Pre-PR Quality Gate

Before each PR:
1. Mutation testing — run `mutation-testing` skill
2. Refactoring assessment — run `refactoring` skill
3. `cargo check`, `cargo test --workspace`, and `cargo clippy` pass
4. `cargo build --target wasm32-unknown-unknown -p frontend` passes (WASM build check — required whenever deps change in `common` or `frontend`)
5. No `any` types, no type assertions

---
*Delete this file when the plan is complete. If `plans/` is empty, delete the directory.*
