# LinguaGuessr — MVP Plan

## Concept

A web-based language identification game inspired by TimeGuessr. Players are shown a short paragraph of text sampled from Wikipedia and must identify what language it is. The goal is to improve language identification skills through repeated play.

## MVP Scope

- 5 languages, mix of scripts (e.g. English, French, Japanese, Arabic, Russian)
- 5 rounds per game session
- Binary scoring (correct / incorrect)
- Score-only end screen (e.g. "You got 3/5")

## Architecture

Single Cargo workspace with three crates:

```
linguaguessr/
├── Cargo.toml          # workspace root
├── backend/            # Axum HTTP server
├── frontend/           # Leptos WASM app
└── common/             # shared types (Language, Round, GameSession, etc.)
```

### Backend (Axum)

- Fetches text samples from the Wikipedia API at game start
- Holds correct answers server-side (untrusted frontend model)
- Serves a REST JSON API
- Proxies Wikipedia so the frontend never calls it directly (enables future pivot to other data sources or static image storage)

### Frontend (Leptos → WASM)

- Compiled to WebAssembly, served as static files
- Calls backend API only — no direct Wikipedia calls
- Reactive quiz UI with combobox input

### Common

- Shared Rust types used by both backend and frontend
- API request/response structs (serialised via serde/JSON)
- `Language` enum with alias mappings (e.g. "FR", "French", "Français" → `Language::French`)

## API Contract

### `GET /api/game`

Start a new game. Backend fetches 5 Wikipedia samples (one per language, randomly selected from the 5), stores the answers, returns:

```json
{
  "game_id": "uuid",
  "rounds": [
    { "round_id": "uuid", "text": "...", "order": 1 },
    ...
  ]
}
```

All 5 rounds returned upfront — one loading screen at game start, no mid-game fetches.

### `POST /api/game/{game_id}/guess`

Submit a guess for a round:

```json
{ "round_id": "uuid", "guess": "French" }
```

Response:

```json
{ "correct": true, "correct_language": "French" }
```

Backend validates the guess against the stored answer. Guess matching is case-insensitive and alias-aware.

## Gameplay Flow

1. Player lands on home screen → clicks "Play"
2. Single loading screen while backend fetches all 5 rounds from Wikipedia
3. Round screen:
   - Styled text block (browser fonts, correct script rendering, RTL for Arabic etc.)
   - Paragraph length: 3–5 sentences
   - Combobox input: type to filter, accepts aliases per language
   - "Submit" button
4. Feedback shown inline: correct ✓ or wrong ✗ (correct answer revealed if wrong)
5. "Next" button advances to the next round (player-controlled pace)
6. After round 5: end screen showing final score (e.g. "3 / 5")
7. "Play again" button starts a new game

## MVP Languages

| Language | Script | Wikipedia subdomain | Aliases |
|----------|--------|---------------------|---------|
| English  | Latin  | en.wikipedia.org    | EN, English |
| French   | Latin  | fr.wikipedia.org    | FR, French, Français |
| Japanese | CJK    | ja.wikipedia.org    | JA, Japanese, 日本語 |
| Arabic   | Arabic | ar.wikipedia.org    | AR, Arabic, العربية |
| Russian  | Cyrillic | ru.wikipedia.org  | RU, Russian, Русский |

## Tech Stack

| Layer | Technology |
|-------|------------|
| Backend | Rust + Axum |
| Frontend | Rust + Leptos (compiled to WASM) |
| Shared types | Rust crate (`common`) |
| Serialisation | serde + serde_json |
| Wikipedia data | Wikipedia REST API (fetched server-side) |
| Deployment | Static frontend files + Axum server |

## Out of Scope for MVP (Future)

- Partial credit / language family scoring
- Real-life images of text (upgrade from styled text block)
- Daily challenge mode
- Streak / endless mode
- Leaderboards
- User accounts
- More than 5 languages
- Share button / social results
- Hard mode (single sentence)
- Difficulty tiers
