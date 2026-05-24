# Plan: Feedback Footer (Phase 6.3)

**Branch**: feat/feedback-footer
**Status**: Active

## Goal

Add a fixed footer bar to every screen with a GitHub repo link on the left and a "Report an issue" button on the right. The button opens a modal where the player can submit feedback. The modal posts to `POST /api/feedback` with the current game context (game_id, round_id, language, article_url) when mid-game.

## Context

- Frontend: Leptos/WASM, `gloo_net` for HTTP, `serde` for serialisation
- Backend endpoint: `POST /api/feedback` (on `feat/feedback-endpoint` branch, not yet merged)
- Leptos components cannot be unit-tested without a browser — testable logic is limited to
  pure functions and data types. Components are verified via `cargo check` + manual browser test.
- Game context threading: `App` holds `GamePhase`; `RoundScreen` manages `round_index`
  internally. To get round_id/language/article_url into the footer, we'll add a
  `RwSignal<Option<RoundContext>>` at the `App` level and have `RoundScreen` write to it
  on every round change.

## Request payload

```json
{
  "message":     "string (required)",
  "email":       "string (optional)",
  "game_id":     "uuid  (optional)",
  "round_id":    "uuid  (optional)",
  "language":    "string (optional)",
  "article_url": "string (optional)"
}
```

## Acceptance Criteria

- [ ] Fixed footer bar visible on every screen (Home, Loading, Playing, Finished, Error)
- [ ] Footer left: link to GitHub repo (`https://github.com/elisedemarie/linguaguessr`)
- [ ] Footer right: "Report an issue" button opens the feedback modal
- [ ] Modal has: message textarea (required), email input (optional), submit button, close button
- [ ] Submitting with an empty message shows a validation error — GitHub not called
- [ ] Valid submission POSTs to `/api/feedback` with message + email (if provided)
- [ ] When mid-game, submission includes game_id, round_id, language, article_url
- [ ] Success state shown after 201 response; modal can then be closed
- [ ] Error state shown on network/server failure
- [ ] Submitting while a request is in flight is disabled

## Slices

Every slice follows RED-GREEN-MUTATE-KILL MUTANTS-REFACTOR. No production code without a failing test.

---

### Slice 1: FeedbackPayload struct + submit_feedback in api.rs

**Value**: The HTTP call is wired up and the payload shape is correct before any UI exists.

**Path**: New `FeedbackPayload` struct (serde Serialize) → `submit_feedback(payload: &FeedbackPayload) -> Result<(), String>` in `api.rs`

**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, `refactoring`.

**Acceptance criteria**:
- `FeedbackPayload` serialises to the expected JSON shape (all optional fields omitted when None)
- `submit_feedback` POSTs to `{BACKEND_URL}/api/feedback` with Content-Type: application/json
- Returns `Ok(())` on 201, `Err(String)` on non-2xx or network failure

**RED**: Unit tests for `FeedbackPayload` serialisation (native, no WASM needed). `cargo check` confirms `submit_feedback` signature compiles.
**GREEN**: Implement struct + function.
**MUTATE**: Run cargo mutants scoped to api.rs.
**KILL MUTANTS**: Address survivors.
**REFACTOR**: Assess only if valuable.
**Done when**: Serialisation tests pass, mutation report reviewed, `cargo check` clean, human approves commit.

---

### Slice 2: FooterBar + FeedbackModal components + App wiring

**Value**: The full end-to-end flow is visible and usable in the browser.

**Path**:
- `RoundContext { game_id, round_id, language, article_url }` — new type in `lib.rs`
- `RwSignal<Option<RoundContext>>` added to `App`, written by `RoundScreen` on each round
- `FooterBar` component: GitHub link + "Report an issue" button
- `FeedbackModal` component: form fields, validation, submit, success/error states
- `App` renders `<FooterBar>` + `<FeedbackModal>` outside the phase switcher (always mounted)
- Modal open/closed state: `RwSignal<bool>` at `App` level, passed to both components

**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, `refactoring`.

**Acceptance criteria** (all verified manually in browser):
- Footer visible on every screen
- GitHub link opens repo in new tab
- "Report an issue" opens modal
- Empty message submit shows inline error, no network call
- Valid submit sends correct payload, shows success message
- Mid-game submit includes game/round context
- Error from server shows error message
- Close button dismisses modal

**RED**: No unit-testable pure logic in this slice — `cargo check` is the compile gate.
**GREEN**: Implement components and wiring.
**MUTATE**: No mutants to run (no pure logic). Confirm `cargo check` passes.
**REFACTOR**: Assess only if valuable.
**Done when**: `cargo check` clean, all browser acceptance criteria met, human approves commit.

---

## Pre-PR Quality Gate

1. `cargo test --workspace` passes
2. `cargo check` passes
3. Manual smoke test:
   - Footer visible on home screen, loading screen, during game, on finished screen
   - Feedback submitted mid-game → GitHub issue created with correct context

---

*Delete this file when the plan is complete. If `plans/` is empty, delete the directory.*
