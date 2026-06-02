# LinguaGuessr

How well can you identify a language just from its text?

Each round shows a real paragraph pulled live from Wikipedia — no translations — and you have to guess the language. The closer your guess, the better your score. Guess Spanish when the answer is Portuguese and you'll still score highly. Scores consider both the language family and how similar the script is.

Five rounds, 75 possible languages, three difficulty levels.

**Play at [linguaguessr.io](https://linguaguessr.io)**

## Modes

- **Play** — pick a difficulty and start immediately.
- **Daily** — one fixed set of languages per day, same for everyone. Come back tomorrow for a new one.
- **Challenge** — generate a shareable link and send it to friends. Anyone who opens it plays the exact same game. Scores are collected anonymously and shown on the finished screen so you can compare.

## Difficulty levels

- **Easy** — 10 of the world's most spoken languages, four multiple-choice options. Correct or nothing.
- **Medium** — 30 languages across diverse scripts and families. Free-text input, partial scoring.
- **Hard** — all 75 languages, including plenty that share scripts or look deceptively similar. Free-text input, partial scoring.

After each round in Medium and Hard, a breakdown shows how close you were on two axes: **Script** and **Family**.

## Languages

75 languages spanning Latin, Cyrillic, Arabic, Devanagari, CJK, Korean, Thai, Georgian, Armenian, Hebrew, Ethiopic, and more — including less commonly featured languages like Burmese, Khmer, Sinhala, Yoruba, and Welsh.

Free-text search matches on ISO codes (`fr`), English names (`french`), and native scripts (`français`, `العربية`, `日本語`).

## Running locally

You'll need [Rust](https://rustup.rs) and [trunk](https://trunkrs.dev).

```bash
# Terminal 1 — backend
cargo run -p backend

# Terminal 2 — frontend
cd frontend && trunk serve
```

Open [http://localhost:8080](http://localhost:8080).

## Stack

Rust all the way down — Axum backend, Leptos frontend compiled to WASM, shared types in a common crate. Single Cargo workspace. Hosted on Cloudflare Pages (frontend) and AWS EC2 (backend).

## Roadmap

- Real-life images of text (street signs, menus, handwriting)
