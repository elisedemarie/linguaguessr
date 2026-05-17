# Plan: Parallel Wikipedia Article Fetching

**Branch**: main
**Status**: Active

## Goal

Fetch all 5 Wikipedia articles concurrently instead of sequentially, reducing game load time from ~sum of 5 fetch times to ~the slowest single fetch.

## Problem

The current `get_game` handler loops over 5 languages and awaits each fetch before starting the next:

```rust
for lang in languages {
    match fetch_article(&lang, ...).await { ... }
}
```

With 5 sequential fetches averaging ~600ms each, a player waits ~3 seconds. Fetching in parallel brings this to ~600ms — the slowest of the 5.

## Approach

Replace the sequential loop with `futures::future::join_all`, firing all 5 fetches simultaneously and collecting results once all have settled.

```rust
let results = futures::future::join_all(
    languages.iter().map(|lang| fetch_article(lang, client))
).await;
```

If any fetch fails, return 503. Otherwise build the `GameSession` from the results.

## New Dependency

`futures = "0.3"` added to `backend/Cargo.toml` (already a transitive dependency of Tokio, but needs to be explicit).

## Acceptance Criteria

- [ ] `GET /api/game` fires all 5 Wikipedia fetches concurrently
- [ ] A single fetch failure still returns 503
- [ ] Response still contains exactly 5 rounds with non-empty text
- [ ] All existing handler tests continue to pass
- [ ] New test: a slow fetch does not block other fetches completing

## Slice

### Slice 1: Replace sequential loop with concurrent fetch in `get_game`

**Value**: Players wait ~600ms to load a game instead of ~3 seconds.

**Path**: `get_game` handler → `join_all` fires 5 `fetch_article` calls simultaneously → collect results → build `GameSession` → return `GameView`.

**Required implementation skills**: `tdd`, `testing`, `mutation-testing`, `refactoring`

**Acceptance criteria**:
- All 5 fetches are initiated before any `await` resolves (verified by test using staggered mock delays)
- Single fetch failure propagates as 503 (same as before)
- Response shape unchanged — 5 rounds, game_id, round_ids

**RED**: Update handler test to verify concurrent behaviour — mock client where one fetch sleeps briefly, assert total time is less than sequential sum would be. Also add a test where one of 5 mocked fetches fails and assert 503.

**GREEN**: Replace `for` loop with `join_all`. Handle `Vec<Result<...>>` — if any is `Err`, return 503; otherwise collect into rounds.

**MUTATE**: Run `cargo mutants -p backend`. Watch for survivors on the error-propagation path (any-fail → 503).

**KILL MUTANTS**: Strengthen tests for survivors.

**REFACTOR**: Assess whether the fetch-and-collect logic should be extracted into a `fetch_all_articles` function for clarity.

**Done when**: All acceptance criteria met, mutation report reviewed, commit approved.

---
*Delete this file when complete.*
