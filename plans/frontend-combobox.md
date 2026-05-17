# Plan: Frontend Combobox UI

**Branch**: feat/frontend-combobox
**Status**: Active

## Goal

A player can open the game in a browser, type into a language combobox, see matching language suggestions in a dropdown, and select one.

## Context

The core filtering logic (`Language::suggestions`) and display labels (`Language::label`) live in `common` and are fully tested. The frontend component is thin rendering logic wired to those functions. The `frontend` crate compiles to WASM via trunk.

## A Note on UI Testing

Browser-side component tests for Leptos WASM require `wasm-bindgen-test` running in headless Chrome — non-trivial to set up and deferred. The behavior being rendered is already covered by 100% mutation-tested logic in `common`. The UI slice is verified visually via `trunk serve`. This is a documented exception.

## Acceptance Criteria

- [ ] `trunk serve` starts and serves the app at `localhost:8080`
- [ ] The page renders a text input with placeholder "Type a language..."
- [ ] Typing `"fr"` shows a dropdown with "French (FR)"
- [ ] Typing `"r"` shows "Russian (RU)"
- [ ] Typing nothing (empty input) shows all 5 languages
- [ ] Clicking a dropdown option sets the input value to that language's label and closes the dropdown
- [ ] Typing an unrecognised string shows no dropdown

## Slices

### Slice 1: Trunk build toolchain (horizontal — unlocks UI development)

**Value**: Unlocks all frontend vertical slices. Without this, no UI can be served.
**Path**: `trunk serve` → compiles `frontend` to WASM → serves `index.html` at `localhost:8080`
**Verification**: `trunk serve` exits without error and the page loads in a browser.

Changes needed:
- Install `trunk` (`cargo install trunk`)
- Add `frontend/index.html` (trunk entry point)
- Add `wasm-bindgen` dep to `frontend/Cargo.toml`
- Update `frontend/src/lib.rs` to mount the App component via `#[wasm_bindgen(start)]`

No TDD for this slice — it's toolchain wiring. Verified by `trunk build` succeeding.

### Slice 2: LanguageCombobox component renders and filters

**Value**: A player can type to filter languages and select one from the dropdown.
**Path**: User types into input → `on:input` handler updates query signal → `Memo` derives `Language::suggestions(query)` → dropdown renders matching options → user clicks option → input shows selected label, dropdown closes.

**Component interface**:
```rust
#[component]
pub fn LanguageCombobox(on_select: Callback<Language>) -> impl IntoView
```

**State**:
- `query: RwSignal<String>` — current input text
- `is_open: RwSignal<bool>` — dropdown visibility

**Derived**:
- `suggestions: Memo<Vec<Language>>` — filtered from `Language::suggestions(&query.get())`

**Verified visually** via `trunk serve`. Core filtering behavior covered by `common` tests.

---
*Delete this file when the plan is complete. If `plans/` is empty, delete the directory.*
