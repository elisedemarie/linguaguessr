# Plan: Home Screen Redesign — Play / Daily Split

**Branch**: feat/home-screen-redesign
**Status**: TODO (not started)

## Goal

Replace the four stacked mode buttons with two primary actions — **Play** and **Daily** — where Play opens a difficulty picker modal, making the hierarchy clearer and the daily challenge feel like a first-class option.

## Motivation

The current home screen has Easy / Medium / Hard / DAILY stacked vertically. As the number of modes grows, this doesn't scale and the difficulty options feel equal to the Daily in visual weight. A two-button layout makes the intent clearer: you either play a regular game (and pick difficulty) or you do today's Daily.

## Rough Design

- Two large buttons: **PLAY** and **DAILY**
- Clicking PLAY opens an inline or overlay difficulty picker (Easy / Medium / Hard)
- Clicking DAILY starts the daily game directly (no further options needed)
- The difficulty picker could be a small modal, a slide-down panel, or inline expansion — TBD

## Notes

- Requires a new `DifficultyPicker` component and a `show_picker` signal on the home screen
- No backend changes needed
- Worth loading `storyboard` skill before implementation to design the modal states

---
*Delete this file when the plan is complete. If `plans/` is empty, delete the directory.*
