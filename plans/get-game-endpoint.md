# Plan: GET /api/game Endpoint

**Branch**: main
**Status**: Active

## Goal

`GET /api/game` returns a `GameView` with 5 rounds of real Wikipedia text. Answers are stored server-side in an in-memory store. This is the first complete backend vertical — a real API call delivers real game content.

## What This Slice Builds

| Thing | Lives in |
|---|---|
| `Round { round_id, text, language }` | `backend/src/game.rs` (backend-only, answer kept private) |
| `GameSession { game_id, rounds }` | `backend/src/game.rs` |
| `GameStore` — `Arc<Mutex<HashMap<Uuid, GameSession>>>` | `backend/src/game.rs` |
| `session_to_view(session) -> GameView` | `backend/src/game.rs` |
| `AppState { store, wikipedia }` | `backend/src/main.rs` |
| `GET /api/game` handler | `backend/src/handlers.rs` |

## Acceptance Criteria

- [ ] `GET /api/game` returns HTTP 200 with a valid JSON `GameView`
- [ ] Response contains exactly 5 rounds
- [ ] Each round has a non-empty `text` and a unique `round_id`
- [ ] Response has a `game_id`
- [ ] The game session is saved in the store (can be looked up after the call)
- [ ] All 5 languages are represented (one round per language)
- [ ] Round order is randomised
- [ ] Wikipedia fetch failure returns HTTP 503

## New Dependencies

- `uuid = { version = "1", features = ["v4", "serde"] }` — backend (common already has it, add to backend)
- `rand = "0.8"` — shuffle language order
- `tower = { version = "0.4", features = ["util"] }` — dev dep for handler testing

## Slices

Every slice follows RED-GREEN-MUTATE-KILL MUTANTS-REFACTOR.

---

### Slice 1: Game session model and view conversion

**Value**: The pure domain logic is tested independently before wiring into HTTP.

**Path**: `Language` × text → `Round` → `GameSession` → `session_to_view` → `GameView`.

**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`

**Acceptance criteria**:
- `session_to_view` strips `language` from each `Round`, exposing only `round_id` and `text`
- Number of rounds in `GameView` equals number of `Round`s in `GameSession`
- `game_id` is preserved across conversion

**RED**: Tests asserting `session_to_view` produces a `GameView` where:
- `game_id` matches the session's `game_id`
- Each `round_id` matches the corresponding `Round`
- `text` matches but no `language` field is present
- `rounds.len()` equals the number of input rounds

**GREEN**: Define `Round`, `GameSession`, implement `session_to_view`.

**MUTATE + KILL MUTANTS + REFACTOR**: Standard cycle.

**Done when**: All criteria met, mutation report reviewed, commit approved.

---

### Slice 2: `GET /api/game` handler

**Value**: A player (or the frontend) can call `GET /api/game` and receive a playable game with real Wikipedia text.

**Path**: HTTP GET → Axum handler → fetch 5 articles via `WikipediaClient` → build `GameSession` → save to `GameStore` → return `GameView` as JSON.

**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`

**Testing approach**: Use `tower::ServiceExt::oneshot` to call the Axum app in tests without a real server. Inject a `MockWikipediaClient` via `AppState` so tests never make real network calls.

**Acceptance criteria**:
- Response is HTTP 200
- Body deserialises to a valid `GameView` with 5 rounds
- All rounds have non-empty text
- Game session is present in the store after the call
- Wikipedia failure → 503 response

**RED**: Tests using `oneshot` with mock Wikipedia client.

**GREEN**: Wire up `AppState`, `GET /api/game` route, handler logic.

**MUTATE + KILL MUTANTS + REFACTOR**: Standard cycle.

**Done when**: All criteria met, mutation report reviewed, commit approved.

---
*Delete this file when complete.*
