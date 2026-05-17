# LinguaGuessr

A language identification game. Read a snippet of real text and guess what language it's written in — across five rounds, with a score at the end.

**Play it at [linguaguessr.pages.dev](https://linguaguessr.pages.dev)**

## How to play

Each round shows you a short paragraph pulled live from Wikipedia. Type a language into the search box, pick from the dropdown, and hit Submit. After 5 rounds you'll see your score.

The game currently includes English, French, Japanese, Arabic, and Russian.

## Running locally

You'll need [Rust](https://rustup.rs) and [trunk](https://trunkrs.dev) installed.

```bash
# Terminal 1 — backend
cargo run -p backend

# Terminal 2 — frontend
cd frontend && trunk serve
```

Then open [http://localhost:8080](http://localhost:8080).

## Roadmap

- More languages
- Real-life images of text (street signs, menus, handwriting)
- Daily challenge mode
- Leaderboard
