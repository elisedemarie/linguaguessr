# LinguaGuessr

A language identification game. Read a real snippet of text pulled from Wikipedia and guess what language it's written in — five rounds, three difficulty levels.

**Play it at [linguaguessr.pages.dev](https://linguaguessr.pages.dev)**

## How to play

Choose a difficulty, then read each round's paragraph and identify the language:

- **Easy** — 10 of the world's most spoken languages, presented as four multiple-choice buttons. Scoring is binary: correct or nothing.
- **Medium** — 30 languages across diverse scripts and families. Free-text search, partial scoring.
- **Hard** — all 75 languages, including many that share scripts or look similar. Free-text search, partial scoring.

In Medium and Hard, you earn points even for a wrong guess if your answer shares a script or language family with the correct one. After each round, a score breakdown shows how your guess compared on two axes: **Script** (0–500) and **Family** (0–500).

Suggestions in free-text mode match on ISO codes (`fr`), English names (`french`), and native scripts (`français`, `العربية`, `日本語`).

## Languages

75 languages spanning Latin, Cyrillic, Arabic, Devanagari, CJK, Korean, Thai, Georgian, Armenian, Hebrew, Ethiopic, and more — including many less commonly featured languages like Burmese, Khmer, Sinhala, Yoruba, and Welsh.

## Tech stack

| Layer | Technology |
|-------|------------|
| Backend | Rust + Axum |
| Frontend | Rust + Leptos (compiled to WASM) |
| Shared types | Rust crate (`common`) |
| Wikipedia data | Wikipedia REST API (fetched server-side) |
| Deployment | Cloudflare Pages (frontend) + Render (backend) |

Single Cargo workspace — `common`, `backend`, `frontend`.

## Running locally

You'll need [Rust](https://rustup.rs) and [trunk](https://trunkrs.dev).

```bash
# Terminal 1 — backend
cargo run -p backend

# Terminal 2 — frontend
cd frontend && trunk serve
```

Open [http://localhost:8080](http://localhost:8080). The frontend talks to `http://localhost:3000` by default.

## Architecture notes

- The backend fetches Wikipedia text and holds correct answers server-side — the client never sees the answer until after it guesses
- Language pool per mode: Easy uses the 10 most spoken languages, Medium uses a curated 30, Hard uses all 75
- Partial scoring uses Jaccard similarity on script family nodes and exact matching on language family hierarchy (sub-branch → branch → family)
- Wikipedia extracts are truncated at 600 characters (char count, not bytes) with retry logic for short articles
- Session store is in-memory — no database, no persistence between server restarts

## Roadmap

- Real-life images of text (street signs, menus, handwriting)
- Daily challenge mode
- Leaderboard
