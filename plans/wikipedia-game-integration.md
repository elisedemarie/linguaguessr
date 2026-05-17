# Plan: Wikipedia Game Integration (Roadmap)

**Status**: Roadmap — individual slices get their own plan files when implementation begins

## Goal

A player can click "Play", see a paragraph of Wikipedia text in an unknown language, type a guess into the combobox, and receive immediate feedback — repeated across 5 rounds with a final score.

## How This Roadmap Works

This file is the high-level sequence. Before implementing any phase, create a focused plan file for that phase (e.g. `plans/api-types.md`). Delete each plan file when its phase is complete. Delete this file when all phases are done.

---

## Wikipedia API

All article fetching uses the Wikipedia REST API summary endpoint:

```
GET https://{lang_code}.wikipedia.org/api/rest_v1/page/random/summary
```

Returns JSON. The `extract` field contains a clean, pre-rendered text summary (no markup). Language codes: `en`, `fr`, `ja`, `ar`, `ru`. No auth required. Backend proxies all requests — frontend never calls Wikipedia directly.

Text trimming: take the first 3 sentences from `extract` (split on `. `). If the extract is fewer than 2 sentences, discard and retry (up to 3 attempts).

---

## Shared Data Model (lives in `common`)

```
GameView          ← returned by GET /api/game (no answers)
  game_id: Uuid
  rounds: Vec<RoundView>

RoundView         ← one round as seen by the client
  round_id: Uuid
  text: String

GuessRequest      ← body of POST /api/game/{id}/guess
  round_id: Uuid
  language: Language

GuessResponse     ← result of a guess
  correct: bool
  correct_language: Language

GameSession       ← backend-only, never sent to client
  game_id: Uuid
  rounds: Vec<Round>

Round             ← backend-only
  round_id: Uuid
  text: String
  language: Language   ← the answer, kept server-side
```

---

## Phases

### Phase 1 — Shared API Types
**Plan file**: `plans/api-types.md` (create when starting)

Add `GameView`, `RoundView`, `GuessRequest`, `GuessResponse` to `common`.
`GameSession` and `Round` live in `backend` only (not shared — client never sees answers).

**Why horizontal**: Both backend endpoints and frontend fetch logic depend on these types. Defining them first means backend and frontend can be built against the same contract without duplication.

**Verification**: `cargo check --workspace` passes. All new types derive `Serialize`/`Deserialize`.

**TDD**: Types have no logic — no tests needed. Verified by compile.

---

### Phase 2 — Wikipedia Article Fetcher
**Plan file**: `plans/wikipedia-fetcher.md` (create when starting)

A backend function `fetch_article(lang: &Language) -> Result<String, FetchError>` that:
- Calls the Wikipedia summary endpoint for the given language
- Extracts the first 3 sentences from the `extract` field
- Returns `Err` if the extract is too short (< 2 sentences) after up to 3 retries

**Dependencies**: `reqwest` (async HTTP client), `serde_json` (already present), `wiremock` (dev dep for HTTP mocking in tests)

**TDD approach**: Define a `WikipediaClient` trait with an async `fetch_summary(url: &str)` method. The real implementation uses `reqwest`. Tests use a `wiremock` mock server. This keeps the fetcher unit-testable without real network calls.

**Key tests**:
- Happy path: mock returns valid JSON → returns trimmed text
- Short extract (< 2 sentences): retries up to 3 times, then returns `Err`
- HTTP error (500): returns `Err`
- All 5 language codes map to correct Wikipedia subdomains

**Why before the endpoint**: The fetcher is a self-contained unit with clear inputs/outputs. Testing it in isolation is far easier than testing it wired into an Axum handler.

---

### Phase 3 — `GET /api/game` Endpoint + In-Memory Store
**Plan file**: `plans/get-game-endpoint.md` (create when starting)

**First real backend vertical.** An API client can `GET /api/game` and receive a `GameView` with 5 rounds of real Wikipedia text.

**In-memory store**: `Arc<Mutex<HashMap<Uuid, GameSession>>>` injected as Axum state. No database. Sessions persist for the lifetime of the process — fine for MVP.

**What this slice does**:
1. Shuffles the 5 languages randomly
2. Fetches one article per language via the Wikipedia fetcher
3. Creates a `GameSession` (answers stored server-side) and saves it to the store
4. Returns `GameView` (no answers) to the client

**TDD approach**: Test the handler with a mock `WikipediaClient` so tests don't make real network calls. Use `axum::test` helpers to call the handler directly.

**Key tests**:
- Response contains exactly 5 rounds
- Each round has a non-empty `text` and a `round_id`
- `game_id` is present in the response
- The game session is stored (can be retrieved from the store after the call)
- Wikipedia fetch failure propagates as a 503

---

### Phase 4 — `POST /api/game/{id}/guess` Endpoint
**Plan file**: `plans/guess-endpoint.md` (create when starting)

**Actor**: Player (via frontend). **Trigger**: Submitting a language guess for a round. **Observable outcome**: `{ correct: true/false, correct_language: Language }`.

**What this slice does**:
1. Looks up the `GameSession` by `game_id`
2. Finds the `Round` by `round_id`
3. Compares `GuessRequest.language` to `Round.language`
4. Returns `GuessResponse`

**Key tests**:
- Correct guess → `{ correct: true, correct_language: French }`
- Wrong guess → `{ correct: false, correct_language: French }`
- Unknown `game_id` → 404
- Unknown `round_id` → 404
- Same round guessed twice → still returns a valid response (idempotent read)

---

### Phase 5 — Frontend Loads a Game and Displays Round Text
**Plan file**: `plans/frontend-game-load.md` (create when starting)

**First frontend-to-backend vertical.** A player clicks "Play", sees a loading state, then sees the text of round 1.

**What this slice does**:
- Adds a "Play" button to the `App` component
- On click: calls `GET /api/game` via `gloo-net` (already a dep via Leptos)
- While loading: shows a loading indicator
- On success: stores the `GameView` in a signal, renders the first round's `text` in a styled text block
- On error: shows an error message

**Does not include**: The combobox for guessing (Phase 6). Just displays text.

**Verified visually** via `trunk serve` + a running backend (`cargo run -p backend`).

---

### Phase 6 — Frontend Submits a Guess and Sees Feedback
**Plan file**: `plans/frontend-guess.md` (create when starting)

**Actor**: Player. **Trigger**: Selecting a language in the combobox and clicking "Submit". **Observable outcome**: "Correct!" or "Wrong — it was French (FR)" shown inline.

**What this slice does**:
- Wires the existing `LanguageCombobox` into the round view
- "Submit" button calls `POST /api/game/{id}/guess` with the selected `Language`
- Shows feedback inline: correct (green) or wrong + correct answer (red)
- "Submit" is disabled until a language is selected

**Verified visually**.

---

### Phase 7 — Round Navigation and End Screen
**Plan file**: `plans/frontend-rounds.md` (create when starting)

**Actor**: Player. **Trigger**: Clicking "Next" after seeing feedback. **Observable outcome**: Player sees the next round's text, or a final score after round 5.

**What this slice does**:
- "Next" button advances a `current_round_index` signal
- After round 5: shows end screen with score (e.g. "3 / 5") and a "Play again" button
- "Play again" resets state and calls `GET /api/game` again

**Verified visually**.

---

## Dependency Order

```
Phase 1 (types)
    ├── Phase 2 (fetcher)
    │       └── Phase 3 (GET /api/game)
    │               └── Phase 4 (POST guess)
    │                       └── Phase 5 (frontend load)
    │                               └── Phase 6 (frontend guess)
    │                                       └── Phase 7 (rounds + end screen)
    └── (frontend types available from Phase 1 onwards)
```

Phases 2–4 are backend-only and can be built and tested without a running frontend. Phase 5 requires both backend (Phase 3) and frontend to be running simultaneously.

---

## Key Dependencies to Add

| Crate | Where | Purpose |
|-------|-------|---------|
| `reqwest` | backend | HTTP client for Wikipedia |
| `wiremock` | backend (dev) | Mock HTTP server for tests |
| `uuid` | backend + common | Game/round IDs |
| `tokio` | backend | Already present |
| `serde_json` | backend | Already present |
| `rand` | backend | Shuffle language order |

---
*Delete this file when all phases are complete.*
