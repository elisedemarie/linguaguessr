# Plan: Frontend Loads a Game and Displays Round Text

**Branch**: main
**Status**: Active

## Goal

A player clicks "Play", sees a loading state, then sees the text of round 1 in a styled block. No guessing yet — just the full fetch-and-display loop working end to end.

## What This Builds

- CORS on the backend (horizontal — without it the browser blocks all cross-origin requests)
- `GamePhase` enum driving the UI state machine: `Home → Loading → Playing → Error`
- `GET /api/game` call from the frontend via `gloo_net`
- Styled text display block for the round excerpt
- Round counter ("Round 1 of 5")

## Acceptance Criteria

- [ ] "Play" button visible on home screen
- [ ] Clicking "Play" shows a loading indicator
- [ ] After load, round 1 text is displayed in a styled block
- [ ] Round counter shows "Round 1 of 5"
- [ ] If backend is unreachable, an error message is shown
- [ ] Backend CORS allows requests from `http://localhost:8080`

## Note on Testing

UI verified visually via `trunk serve` + `cargo run -p backend`. Game fetch logic is thin orchestration over already-tested types. CORS is verified by the browser not blocking requests.

## Slices

### Slice 1: CORS middleware on backend (horizontal)

**Verification**: Browser devtools shows no CORS errors when frontend calls backend.

Add `tower-http` with `cors` feature to backend. Allow all origins in development.

### Slice 2: Frontend game state machine and round display

**Path**: Click "Play" → `spawn_local` fires `GET /api/game` → `GamePhase` signal transitions `Home → Loading → Playing(GameView)` → round text rendered in styled block.

**Backend URL**: Hardcoded `http://localhost:3000` for now.

**Done when**: Full loop visible in browser, commit approved.

---
*Delete this file when complete.*
