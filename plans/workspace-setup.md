# Plan: Cargo Workspace Setup

**Branch**: feat/workspace-setup
**Status**: Active

## Goal

Restructure the repo into a Cargo workspace with three crates (`common`, `backend`, `frontend`) so all future vertical slices have a clean, buildable foundation to work from.

## Why This Is a Horizontal Slice

This is an intentional horizontal exception. It is the minimum scaffolding needed before any vertical slice can be written — there is no meaningful game behavior to deliver until the workspace compiles. It is independently verifiable (`cargo build --workspace` passes) and introduces zero unused abstractions.

## Acceptance Criteria

- [ ] `cargo build --workspace` succeeds with no errors
- [ ] `cargo test --workspace` runs (zero tests is fine — just must not fail)
- [ ] `cargo check --workspace` passes
- [ ] `backend`, `frontend`, and `common` are separate crates within the workspace
- [ ] `common` is listed as a dependency in both `backend` and `frontend`
- [ ] The old `src/main.rs` is removed

## Target Directory Structure

```
linguaguessr/
├── Cargo.toml              # workspace root (no [package], just [workspace])
├── Cargo.lock
├── plans/
│   ├── mvp.md
│   └── workspace-setup.md
├── common/
│   ├── Cargo.toml          # lib crate
│   └── src/
│       └── lib.rs          # placeholder (pub mod types;)
├── backend/
│   ├── Cargo.toml          # bin crate, depends on common
│   └── src/
│       └── main.rs         # placeholder (axum stub — listens, returns 200)
└── frontend/
    ├── Cargo.toml          # lib crate (cdylib for WASM), depends on common
    └── src/
        └── lib.rs          # placeholder (leptos app stub)
```

## Crate Manifests

### Root `Cargo.toml`

```toml
[workspace]
members = [
    "common",
    "backend",
    "frontend",
]
resolver = "2"
```

### `common/Cargo.toml`

```toml
[package]
name = "common"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = { version = "1", features = ["derive"] }
```

### `backend/Cargo.toml`

```toml
[package]
name = "backend"
version = "0.1.0"
edition = "2024"

[dependencies]
common = { path = "../common" }
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

### `frontend/Cargo.toml`

```toml
[package]
name = "frontend"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
common = { path = "../common" }
leptos = { version = "0.7", features = ["csr"] }
```

## Placeholder Source Files

### `common/src/lib.rs`

```rust
pub mod types;
```

### `common/src/types.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    French,
    Japanese,
    Arabic,
    Russian,
}
```

### `backend/src/main.rs`

```rust
#[tokio::main]
async fn main() {
    let app = axum::Router::new();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### `frontend/src/lib.rs`

```rust
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! { <p>"LinguaGuessr"</p> }
}
```

## Slice

### Slice 1: Cargo workspace builds cleanly with three placeholder crates

**Value**: Every future slice has a compilable workspace to build on.
**Path**: Root Cargo.toml → workspace resolver → three member crates → shared `common` dep → `cargo build --workspace` exits 0.
**Required implementation skills**: No TDD needed — this is pure scaffolding with no logic. Verification is `cargo build --workspace` and `cargo check --workspace`.
**Acceptance criteria**:
  - `cargo build --workspace` exits 0
  - `cargo check --workspace` exits 0
  - `cargo test --workspace` exits 0
  - `common::types::Language` is importable from `backend` and `frontend`
**Done when**: All three checks pass, human approves commit.

## Post-Setup Notes

- `trunk` (WASM bundler) will be needed to build and serve the frontend — not configured here, deferred to the first frontend vertical slice
- `cargo-leptos` is an alternative to trunk for Leptos projects — evaluate when the first frontend slice begins
- The `frontend` crate uses `csr` (client-side rendering) feature for now; SSR can be added later if needed

---
*Delete this file when the plan is complete. If `plans/` is empty, delete the directory.*
