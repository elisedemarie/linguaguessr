# Plan: Frontend Submits a Guess and Sees Feedback

**Branch**: main
**Status**: Active

## Goal

Player selects a language in the combobox, clicks Submit, and sees immediate inline feedback — green "Correct!" or red "Wrong — it was French (FR)".

## Acceptance Criteria

- [ ] Submit button is disabled until a language is selected from the dropdown
- [ ] Clicking Submit calls `POST /api/game/:game_id/guess`
- [ ] Correct guess shows green "✓ Correct!" feedback
- [ ] Wrong guess shows red "✗ Wrong — it was [language]" feedback
- [ ] Combobox and Submit button hide after submission (feedback replaces them)
- [ ] Submit button shows "Submitting..." while the request is in flight

## What Changes

- `RoundScreen` gains: `selected: RwSignal<Option<Language>>`, `feedback: RwSignal<Option<GuessResponse>>`, `submitting: RwSignal<bool>`
- `LanguageCombobox.on_select` sets `selected`
- Submit button wired to `post_guess_to_backend` async fn
- Feedback rendered conditionally below the text block
- CSS for `.feedback.correct` and `.feedback.wrong`

## Note on Testing

Verified visually. Core types and backend validation already tested.

---
*Delete this file when complete.*
