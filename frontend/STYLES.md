# LinguaGuessr — Frontend Style Guide

## Theme

Warm, dark, autumnal. Think old libraries, ink, parchment, candlelight. The palette is espresso browns, amber gold, and earthy accents. Nothing should feel cool-toned, clinical, or tech-generic.

---

## Colour Palette

All colours are defined as CSS variables in `style.css`. Never hardcode a hex value in a rule — always use a variable.

### Backgrounds

| Variable | Value | Use |
|---|---|---|
| `--bg` | `#1a1714` | Page background. The darkest layer. |
| `--surface` | `#242018` | Cards, inputs, dropdowns — one step above bg. |
| `--surface-raised` | `#2e2820` | Elevated surfaces like tooltips. |

### Borders

| Variable | Value | Use |
|---|---|---|
| `--border` | `#3d3628` | Standard border on inputs, cards, buttons. |
| `--border-soft` | `#2e2820` | Subtle dividers, disabled states. |

### Text

| Variable | Value | Use |
|---|---|---|
| `--text` | `#e8ddd0` | Primary text. Warm off-white, not pure white. |
| `--muted` | `#7a6e60` | Secondary text, labels, placeholders. |
| `--faint` | `#4a4038` | Disabled text. |

### Accent — Gold

The primary accent. Used for interactive elements, highlights, and the score display.

| Variable | Value | Use |
|---|---|---|
| `--gold` | `#c9963a` | Default gold. Buttons, focus rings, round counter. |
| `--gold-bright` | `#e0ad4a` | Hover states, score display. |
| `--gold-dim` | `rgba(201,150,58,0.15)` | Selected/hovered backgrounds. |
| `--gold-on` | `#1a1208` | Text colour when sitting on a gold background. |

### Feedback — Correct

Warm sage green. Not mint, not lime. Should feel like "good" without screaming.

| Variable | Value | Use |
|---|---|---|
| `--green` | `#86c98a` | Text and borders in correct feedback. |
| `--green-bg` | `rgba(134,201,138,0.07)` | Background tint of correct feedback card. |
| `--green-border` | `rgba(134,201,138,0.25)` | Border of correct feedback card. |

### Feedback — Wrong

Warm terracotta/rust. Not alarm red. Should feel like "miss" not "error".

| Variable | Value | Use |
|---|---|---|
| `--red` | `#d4726a` | Text and borders in wrong feedback. |
| `--red-bg` | `rgba(212,114,106,0.07)` | Background tint of wrong feedback card. |
| `--red-border` | `rgba(212,114,106,0.25)` | Border of wrong feedback card. |

### Special — Parchment

Used exclusively for the Wikipedia text block — the hero element of each round.

| Variable | Value | Use |
|---|---|---|
| `--parchment` | `#fdf8f0` | Text block background. Cream, not white. |
| `--parchment-text` | `#251d14` | Text on parchment. Dark warm brown. |

---

## Typography

Two fonts are loaded via Google Fonts in `index.html`.

### Playfair Display
- **When:** Titles only. `h1.title` on the home screen, "Game over!" on the finished screen.
- **Weight:** 900 for the page title, 700 elsewhere.
- **Never use for:** Body text, UI labels, buttons, numbers. High stroke contrast makes it hard to read at small sizes or in numerals.

### Inter
- **When:** Everything else. Buttons, labels, inputs, body copy, UI text.
- **Weight:** 400 (body), 500 (secondary labels), 600 (buttons, axis names), 700 (score numbers).

### Georgia (system)
- **When:** The final score number on the finished screen. Sits between Playfair (too decorative) and Inter (too plain) for numerals specifically.
- **Never use:** Anywhere else. It's a deliberate exception for one specific use case.

### Do not introduce new fonts without good reason. Three is already enough.

---

## Difficulty Button Palette

The three mode buttons use a tonal progression — same dark-card approach, different earthy hues. They read light-gold → amber → rust, implying difficulty without using traffic-light colours.

| Button | Background | Text | Feel |
|---|---|---|------|
| Easy | `#2e2618` | `#c4a96a` | Warm parchment-gold, muted and welcoming |
| Medium | `#3a2e10` | `#c9963a` | Amber, our main gold |
| Hard | `#2e1e14` | `#c07a5a` | Rust/clay, warmer and more intense |

**Do not use green for Easy.** Green is reserved for correct feedback and the two meanings would conflict.

---

## Feedback States

Feedback cards use a 3px left accent border and a very faint tinted background. They do not use heavy fills.

```
border-left: 3px solid <colour>
border: 1px solid <border-colour>   ← the other three sides, softer
background: <bg-colour>             ← 7% opacity tint only
```

This keeps the feedback readable against the dark background without the card overwhelming the score breakdown content inside it.

---

## The Text Block

The parchment text block is the visual centrepiece of each round. Treat it accordingly.

- Background is always `--parchment` — never dark, never transparent
- Left border is always `--gold` (3px) — marks it as the "reading surface"
- Generous padding (`1.75rem 2rem`) — the text needs room to breathe
- Deep shadow (`0 8px 32px rgba(0,0,0,0.5)`) — lifts it off the dark background
- Font stays in the body font stack — Wikipedia text comes in every script and a serif may not render all of them correctly

---

## Spacing & Layout

- Page max-width: `640px`, centred
- Base gap between stacked elements in a screen: `1.25rem`
- Base gap inside component groups (e.g. buttons): `0.5rem–0.75rem`
- Standard border-radius: `--radius` (10px) for cards, `--radius-sm` (6px) for buttons and inputs
- No hardcoded pixel values for spacing — use `rem`

---

## What to Avoid

- **No cool greys or blues.** Every neutral should read warm. If something looks blue-grey, it's wrong.
- **No pure white or pure black.** `--text` and `--bg` are deliberately off.
- **No green outside feedback.** It reads as "correct" in this game.
- **No Playfair Display on numbers.** Its thin strokes disappear at gold-on-dark.
- **No new accent colours.** Gold, sage, rust — that's the palette. Adding a fourth accent breaks the theme.
- **No hardcoded hex values** outside of `:root`. Everything goes through a variable.
