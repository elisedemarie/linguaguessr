# Plan: POST /api/game/{id}/guess Endpoint

**Branch**: main
**Status**: Active

## Goal

`POST /api/game/:game_id/guess` validates a player's language guess against the stored answer and returns whether it was correct, plus the correct language.

## API

**Request**: `POST /api/game/:game_id/guess`
```json
{ "round_id": "<uuid>", "language": "French" }
```

**Response 200**:
```json
{ "correct": true, "correct_language": "French" }
```

**Response 404**: game_id not found, or round_id not found within that game.

## Acceptance Criteria

- [ ] Correct guess → 200 `{ correct: true, correct_language: "French" }`
- [ ] Wrong guess → 200 `{ correct: false, correct_language: "French" }`
- [ ] Unknown `game_id` → 404
- [ ] Unknown `round_id` (game exists, round doesn't) → 404
- [ ] Guessing the same round twice returns a valid response (idempotent read)

## Slice

### Slice 1: `POST /api/game/:game_id/guess` handler

**Value**: The backend can validate a player's guess server-side without ever exposing the answer to the client.

**Path**: HTTP POST with JSON body → extract `game_id` from path → look up `GameSession` in store → find `Round` by `round_id` → compare `language` → return `GuessResponse`.

**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`

**Testing approach**: Pre-populate the store with a known `GameSession` before each test. Use `tower::ServiceExt::oneshot` to send requests.

**RED**: Tests for correct guess, wrong guess, unknown game_id, unknown round_id, idempotent second guess.

**GREEN**: Add `post_guess` handler to `handlers.rs`, wire route in `main.rs`.

**MUTATE + KILL + REFACTOR**: Standard cycle. Watch for mutants on the `==` comparison (correct vs always-true/always-false).

**Done when**: All criteria met, mutation report reviewed, commit approved.

---
*Delete this file when complete.*
