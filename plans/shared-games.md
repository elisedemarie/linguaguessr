# Plan: Shared Games (Seed-Based Multiplayer)

**Branch**: feat/shared-games
**Status**: Active

## Goal

Players can create a shareable game via a short seed code; anyone with the code plays the exact same set of languages and articles. Later slices add an anonymous leaderboard so players can compare scores.

## Design Decisions

- **Seed format**: 6-character alphanumeric code (`AB12CD`), ~2.1B combinations. Uppercase only for readability.
- **Seed is client-side generated**: frontend picks a random code before calling the API. No round-trip needed to "create" a game. The code is just passed as a query param.
- **Seed → u64**: hash the code string to a u64 using a stable algorithm (FNV-1a or similar), then feed into ChaCha8Rng — same approach as daily.
- **Seed + mode together define the game**: seeded games respect the `mode` param as normal — Easy pool for Easy, Medium pool for Medium, full pool for Hard. Same seed on different modes gives different language selections, which is expected.
- **Seed is orthogonal to GameMode**: no new variant needed. When `seed` is present the backend uses deterministic selection within whatever pool `mode` dictates; scoring rules follow `mode` as normal. Default mode is Medium if omitted.
- **Share mechanic**: URL param `?seed=AB12CD`. Sharing = copying the URL. No custom code-entry UI in slice 1.
- **Leaderboard is in-memory only** — lost on server restart. Acceptable for now.

## Acceptance Criteria (full feature)

- [ ] `GET /api/game?seed=AB12CD` returns the same 5 languages every time for that code
- [ ] Two players using the same seed play identical games (same languages, same articles)
- [ ] Home screen has a "Share a game" button that generates a code and starts that game
- [ ] Finished screen for a seeded game shows the seed code and a copy-link button
- [ ] Opening `/?seed=AB12CD` in a browser auto-starts that seeded game (no home screen)
- [ ] After finishing a seeded game, score is posted to the backend
- [ ] When starting a seeded game someone else created, the finished screen shows how many others played and their average score

## Slices

Every slice follows RED-GREEN-MUTATE-KILL MUTANTS-REFACTOR. No production code without a failing test.

---

### Slice 1: Deterministic language selection from an arbitrary seed string

**Value**: Unlocks slice 2 — the backend needs this pure function to serve seeded games.
**Path**: `common` crate only — pure function `Language::seeded_languages(seed: &str) -> [Language; 5]`. Hashes the string to u64, uses ChaCha8Rng (same as `daily_languages`). Draws from the full pool (all 75 languages), no repeats.
**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`
**Acceptance criteria**:
- Same seed string → same 5 languages, every time
- Different seed strings → (almost always) different results
- Result is always exactly 5 distinct languages, all from `Language::all()`
- Does not touch backend or frontend

**RED**: Tests in `common/src/types.rs` — same-seed+pool determinism, all-distinct invariant, different-seed divergence, results are a subset of the given pool.
**GREEN**: Add `seeded_languages(seed: &str, pool: &[Language]) -> [Language; 5]` — hash seed to u64 with FNV-1a (no new dep needed: manual 8-line impl), feed into `ChaCha8Rng::seed_from_u64`, shuffle-pick 5 from the given pool.
**MUTATE**: Run `cargo mutants -p common`.
**KILL MUTANTS**: Address survivors.
**REFACTOR**: Assess deduplication with `daily_languages` (both use same rng pattern — consider extracting a shared private fn).
**Done when**: All tests pass, mutation report reviewed, human approves commit.

---

### Slice 2: Backend serves a deterministic game for `?seed=AB12CD`

**Value**: API client (frontend, curl) can call `GET /api/game?seed=AB12CD` and get the same game back every time.
**Path**: `GET /api/game` query params → `select_languages` → `GameSession` stored in-memory → `GameView` returned. Add optional `seed: Option<String>` to `GameParams`. When present, call `Language::seeded_languages(&seed, language_pool(&mode))` instead of the random path.
**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`
**Acceptance criteria**:
- `GET /api/game?seed=AB12CD` returns a valid `GameView` with 5 rounds (defaults to Medium pool)
- `GET /api/game?seed=AB12CD&mode=easy` uses the Easy pool (10 languages)
- `GET /api/game?seed=AB12CD&mode=hard` uses the full pool (75 languages)
- Same seed + same mode → same languages every time
- Same seed + different mode → different language selection (different pool)
- Existing mode-based tests are unaffected

**RED**: Integration tests in `backend/src/handlers.rs` — seeded game is created successfully; two sessions with the same seed have the same correct languages.
**GREEN**: Add `seed` field to `GameParams`; update `select_languages` to branch on `seed.is_some()`.
**MUTATE**: Run `cargo mutants -p backend`.
**KILL MUTANTS**: Address survivors.
**REFACTOR**: Assess.
**Done when**: All tests pass, WASM build check passes, mutation report reviewed, human approves commit.

---

### Slice 3: Home screen "Share a game" button starts a seeded game

**Value**: User can click one button, get a seeded game, and see their seed code on the finished screen so they can share it.
**Path**: Home screen → user clicks "Share" → frontend generates random 6-char code → `fetch_game(seed=CODE)` → `GamePhase::Playing { seed: Some(CODE), .. }` → game plays normally → `GamePhase::Finished { seed: Some(CODE), .. }` → finished screen shows code + copy-URL button.
**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`
**Acceptance criteria**:
- "Share a game" button appears on home screen
- Clicking it generates a random uppercase 6-char alphanumeric code, calls the seeded API with the currently selected difficulty, and starts the game
- Finished screen (seeded game only) shows: the seed code in a prominent box, and a "Copy link" button that copies `<origin>/?seed=CODE&mode=MODE` to clipboard
- Non-seeded games (regular/daily) are unchanged

**RED**: Unit tests for the seed-generation function (correct length, correct charset). For the Leptos rendering: since it's UI-only, explain to human and ask permission to skip RED for rendering.
**GREEN**: Add `seed` field to `GamePhase` variants; add `generate_seed()` fn; wire up button and finished screen display.
**MUTATE**: `cargo mutants -p frontend` (logic only — rendering mutations are low value).
**KILL MUTANTS**: Address survivors on logic functions.
**REFACTOR**: Assess.
**Done when**: All tests pass, WASM build check passes, mutation report reviewed, human approves commit.

---

### Slice 4: Opening `/?seed=AB12CD` auto-starts the seeded game

**Value**: A player who receives a shared link lands directly in the game — no home screen, no code entry.
**Path**: App startup → read `window.location.search` for `?seed=` and `?mode=` params → if seed present, skip `GamePhase::Home`, immediately call `fetch_game(seed=CODE, mode=MODE)` → `GamePhase::Loading` → game plays normally.
**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`
**Acceptance criteria**:
- `/?seed=AB12CD` skips the home screen and starts that seeded game directly
- An invalid/unknown seed still creates a game (backend just uses whatever hash it gets — no validation needed)
- No URL param → home screen as normal
- After finishing, the finished screen still shows the seed code and copy-link button (from slice 3)

**RED**: Unit test for the URL-param parsing function (extracts seed, returns None when absent, handles malformed input).
**GREEN**: Add URL param reading in `App` component startup (WASM `window.location.search`); branch on presence of seed.
**MUTATE**: `cargo mutants -p frontend`.
**KILL MUTANTS**: Address survivors.
**REFACTOR**: Assess.
**Done when**: All tests pass, WASM build check passes, mutation report reviewed, human approves commit.

---

### Slice 5: Backend stores scores for seeded games (in-memory)

**Value**: Unlocks slice 6 — score data must exist before it can be displayed.
**Path**: Add `seed_scores: Arc<Mutex<HashMap<String, Vec<u32>>>>` to `AppState`. `POST /api/seeds/:seed/scores` body `{ score: u32 }` → appends to the vec. `GET /api/seeds/:seed/scores` → returns `{ scores: [u32] }` — full anonymous list, no attribution. Horizontal slice: named here because slice 6 depends directly on it, and it has independent verification via HTTP tests.
**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`
**Acceptance criteria**:
- `POST /api/seeds/AB12CD/scores` with `{ "score": 3000 }` returns 200
- `GET /api/seeds/AB12CD/scores` returns `{ "scores": [3000] }` after one post
- Multiple posts accumulate into the list in order
- Unknown seed returns `{ "scores": [] }` (not 404)

**RED**: Integration tests for both endpoints.
**GREEN**: Add `seed_scores` to state, add two route handlers.
**MUTATE**: `cargo mutants -p backend`.
**KILL MUTANTS**: Address survivors.
**REFACTOR**: Assess.
**Done when**: All tests pass, mutation report reviewed, human approves commit.

---

### Slice 6: Finished screen shows leaderboard for seeded games

**Value**: Player finishing a shared game sees a list of all scores others have posted for that challenge — social proof that closes the loop.
**Path**: `GamePhase::Finished` (seeded) → frontend `POST /api/seeds/:seed/scores` on mount → then `GET /api/seeds/:seed/scores` → display the score list in the finished screen panel.
**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`
**Acceptance criteria**:
- Score is posted automatically when a seeded game finishes (once only — no duplicate on re-render)
- Finished screen shows the list of all scores for that seed (anonymous, no attribution)
- If fetch fails (network error), the leaderboard section is hidden gracefully — rest of finished screen unaffected
- Non-seeded games do not post or display leaderboard data

**RED**: Unit tests for the API call logic; rendering is UI-only — explain and ask permission to skip RED for that part.
**GREEN**: Add `api::post_seed_score` and `api::fetch_seed_scores`; wire into finished screen.
**MUTATE**: `cargo mutants -p frontend`.
**KILL MUTANTS**: Address survivors.
**REFACTOR**: Assess.
**Done when**: All tests pass, WASM build check passes, mutation report reviewed, human approves commit.

---

## Pre-PR Quality Gate

Before each PR:
1. `cargo test --workspace`
2. `cargo build --target wasm32-unknown-unknown -p frontend`
3. Mutation testing report reviewed
4. Typecheck passes (Rust — covered by build)

---

*Delete this file when the plan is complete. If `plans/` is empty, delete the directory.*
