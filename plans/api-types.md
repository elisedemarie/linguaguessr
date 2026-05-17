# Plan: Shared API Types

**Branch**: main
**Status**: Active

## Goal

Define the shared request/response types in `common` that both backend and frontend compile against, so the API contract is a single source of truth with no duplication.

## Types

| Type | Direction | Lives in |
|------|-----------|----------|
| `GameView` | Backend → Client | `common` |
| `RoundView` | Backend → Client | `common` |
| `GuessRequest` | Client → Backend | `common` |
| `GuessResponse` | Backend → Client | `common` |
| `GameSession` | Backend only | `backend` (Phase 3) |
| `Round` | Backend only | `backend` (Phase 3) |

## Acceptance Criteria

- [ ] `GameView { game_id: Uuid, rounds: Vec<RoundView> }` serialises to JSON
- [ ] `RoundView { round_id: Uuid, text: String }` serialises to JSON
- [ ] `GuessRequest { round_id: Uuid, language: Language }` deserialises from JSON
- [ ] `GuessResponse { correct: bool, correct_language: Language }` serialises to JSON
- [ ] `Language` serialises as a plain string (e.g. `"French"`) not a serde enum object
- [ ] All types round-trip (serialise → deserialise → same value)
- [ ] `cargo check --workspace` passes — both `backend` and `frontend` can import the types

## Slice

### Slice 1: API types defined in `common` with serde round-trip tests

**Value**: Backend and frontend share a single, tested API contract with no type duplication.

**Path**: New `common/src/api.rs` module → exported from `common/src/lib.rs` → imported in `backend` and `frontend` Cargo.toml (already depend on `common`).

**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`

**Acceptance criteria**: All four types compile, serialise, and deserialise correctly. Serde field names match what the API will send over the wire (`game_id`, `round_id`, `correct_language` — all snake_case).

**RED**: Write failing tests for:
- `RoundView` serialises to `{"round_id": "<uuid>", "text": "..."}`
- `GameView` serialises to `{"game_id": "<uuid>", "rounds": [...]}`
- `GuessRequest` deserialises from `{"round_id": "<uuid>", "language": "French"}`
- `GuessResponse` serialises with `correct: true` and `correct: false` variants
- `Language` serialises as `"French"` not `{"French": null}` (confirm serde default)
- Full round-trip for each type

**GREEN**: Add `uuid` to `common/Cargo.toml` (with `serde` feature). Create `common/src/api.rs` with the four structs, all deriving `Debug, Clone, PartialEq, Serialize, Deserialize`.

**MUTATE**: Run `cargo mutants -p common`. These are data types with no branching logic — expect mostly unviable mutants. Any survivors on serde field name assertions are worth killing.

**KILL MUTANTS**: Address survivors.

**REFACTOR**: Assess whether types belong in `api.rs` or alongside `Language` in `types.rs`. Separate file is cleaner since it's a distinct concern.

**Done when**: All acceptance criteria met, mutation report reviewed, commit approved.

---
*Delete this file when complete.*
