# Plan: Home Screen Redesign — Play / Daily Split

**Branch**: feat/home-screen-redesign
**Status**: Active

## Goal

Replace the four stacked mode buttons with two primary actions — **PLAY** and **DAILY** — where PLAY expands inline to reveal Easy / Medium / Hard, making the mode hierarchy clear and Daily a true first-class option.

## Motivation

Easy/Medium/Hard are *settings for the same action*. Daily is a *different action entirely*. The current layout treats all four as equals, which buries that distinction and doesn't scale as modes are added.

## Design

### Layout

Two options to try (build both, pick the best):

**Option A — Stacked (recommended)**
```
┌──────────────────────────────┐
│           PLAY               │  ← large primary button
└──────────────────────────────┘
  ┌────────┐┌────────┐┌────────┐   ← expands below when PLAY pressed
  │  Easy  ││ Medium ││  Hard  │
  └────────┘└────────┘└────────┘
┌──────────────────────────────┐
│           DAILY              │  ← or played state (emojis + score)
└──────────────────────────────┘
```

**Option B — Side by side**
```
┌──────────────┐ ┌──────────────┐
│     PLAY     │ │    DAILY     │
└──────────────┘ └──────────────┘
┌────────┐┌────────┐┌────────┐     ← expands below PLAY when pressed
│  Easy  ││ Medium ││  Hard  │
└────────┘└────────┘└────────┘
```

### Behaviour
- PLAY starts **closed** (difficulty picker hidden)
- Pressing PLAY toggles the difficulty picker open/closed
- Pressing Easy/Medium/Hard starts the game immediately (no extra confirm)
- DAILY button behaviour unchanged — starts daily directly, or shows played state
- No modal/overlay — picker expands inline below the PLAY button

### Visual treatment
- PLAY and DAILY: same size, same visual weight as current mode buttons
- Difficulty pills: smaller, secondary style — clearly subordinate to PLAY/DAILY
- Picker appearance: fade + slide-down, ~150ms
- PLAY button gets a subtle "open" indicator (chevron or slight style change) when expanded

## Slices

Every slice follows RED-GREEN-MUTATE-KILL MUTANTS-REFACTOR. No production code without a failing test.

### Slice 1: HomeScreen logic — PLAY toggle + difficulty picker dispatch

**Value**: The new component structure exists and is testable before any styling.
**Path**: `HomeScreen` component → internal `show_picker: RwSignal<bool>` → conditional render of difficulty pill row.
**Required implementation skills**: Load `tdd`, `testing`, `mutation-testing`, and `refactoring` before code changes.
**Acceptance criteria**:
- [ ] `show_picker` is internal to `HomeScreen` (not a prop), starts false
- [ ] When `show_picker` is false, only PLAY and DAILY buttons are visible (no Easy/Medium/Hard)
- [ ] When `show_picker` is true, Easy/Medium/Hard pills are visible
- [ ] Clicking PLAY toggles `show_picker`
- [ ] Clicking a difficulty pill calls `on_play` with the correct `GameMode` and sets `show_picker` to false
- [ ] Existing Daily button behaviour unchanged
- [ ] All tests pass, mutation report clean

**RED**: Unit tests for picker toggle logic and `on_play` dispatch via difficulty pill.
**GREEN**: Refactor `HomeScreen` — remove `mode-buttons` div, add PLAY toggle + conditional picker row.
**MUTATE**: Run mutation testing on `home_screen.rs`.
**KILL MUTANTS**: Address survivors.
**REFACTOR**: Consider extracting a `DifficultyPicker` component if it helps readability.
**Done when**: Acceptance criteria met, mutation report reviewed, human approves commit.

### Slice 2: CSS — style PLAY/DAILY buttons and difficulty picker

**Value**: The redesigned home screen looks polished and matches the warm autumn palette.
**Path**: `style.css` — new classes for PLAY/DAILY primary buttons, difficulty pill row, picker open/close animation.
**Required implementation skills**: CSS only. Visual review replaces mutation testing.
**Acceptance criteria**:
- [ ] PLAY and DAILY buttons: same width, prominent, warm autumn palette
- [ ] Difficulty pills: smaller, secondary style — clearly subordinate to PLAY/DAILY
- [ ] Picker fades + slides in (~150ms) when PLAY is pressed
- [ ] PLAY button has a subtle visual indicator when picker is open
- [ ] Layout works on mobile (max-width 640px) and on desktop
- [ ] Daily played-state display (emojis + score) fits naturally in the new layout
- [ ] No visual regressions on other screens

**RED**: Visual inspection — build both Option A (stacked) and Option B (side-by-side), pick the best.
**GREEN**: Write CSS for the chosen layout.
**MUTATE**: N/A for CSS — visual review only.
**REFACTOR**: Remove redundant style rules from old `.mode-buttons` block.
**Done when**: Acceptance criteria met (visual review), human approves commit.

## Pre-PR Quality Gate

Before PR:
1. `cargo test --workspace` — all tests pass
2. `cargo build --target wasm32-unknown-unknown -p frontend` — WASM build clean
3. Mutation testing — `cargo mutants -p frontend`
4. `cargo check --workspace`
5. Visual review in browser (`trunk serve`)

---
*Delete this file when the plan is complete. If `plans/` is empty, delete the directory.*
