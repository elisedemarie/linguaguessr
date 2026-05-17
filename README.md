# LinguaGuessr

A language identification game built entirely in Rust. Read a snippet of real Wikipedia text and guess what language it's written in.

Inspired by [TimeGuessr](https://timeguessr.com). Built to improve language recognition skills through repeated play.

## How it works

Each round shows you a paragraph fetched live from Wikipedia in one of five languages. Type your guess into the search box, pick from the dropdown, and hit Submit. After 5 rounds you get your score.

**Languages in the MVP:** English, French, Japanese, Arabic, Russian

The answer is never sent to the browser — guesses are validated server-side.

## Tech stack

| Layer | Technology |
|---|---|
| Backend | Rust · [Axum](https://github.com/tokio-rs/axum) |
| Frontend | Rust · [Leptos](https://leptos.dev) (compiled to WebAssembly) |
| Shared types | Rust · `common` crate |
| Content | [Wikipedia REST API](https://en.wikipedia.org/api/rest_v1/) |
| WASM bundler | [Trunk](https://trunkrs.dev) |

Everything — backend, frontend, and shared types — lives in a single Cargo workspace.

## Running locally

You need two terminals.

**Prerequisites**
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

**Terminal 1 — backend** (runs on port 3000)
```bash
cargo run -p backend
```

**Terminal 2 — frontend** (runs on port 8080)
```bash
cd frontend && trunk serve
```

Then open [http://localhost:8080](http://localhost:8080).

## Project structure

```
linguaguessr/
├── Cargo.toml          # workspace root
├── common/             # shared types (Language, GameView, GuessRequest, …)
├── backend/            # Axum API server
│   └── src/
│       ├── main.rs
│       ├── handlers.rs # GET /api/game, POST /api/game/:id/guess
│       ├── game.rs     # GameSession, Round, session_to_view
│       └── wikipedia.rs# Wikipedia fetcher with retry + 600-char truncation
└── frontend/           # Leptos WASM app
    ├── index.html
    ├── style.css
    └── src/lib.rs
```

## API

| Endpoint | Description |
|---|---|
| `GET /api/game` | Fetch 5 Wikipedia rounds (answers stored server-side) |
| `POST /api/game/:id/guess` | Submit a language guess, receive correct/wrong + answer |

## Roadmap

- [ ] More languages
- [ ] Real images of text (street signs, menus, handwriting)
- [ ] Partial credit for close guesses (language families)
- [ ] Daily challenge mode
- [ ] Streak / endless mode
- [ ] Leaderboard
- [ ] Difficulty tiers (single sentence = hard mode)
