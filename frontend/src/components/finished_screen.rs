use common::types::GameMode;
use leptos::prelude::*;

use crate::score::{display_score, max_score};

#[component]
pub fn FinishedScreen(
    score: u32,
    mode: GameMode,
    on_play_again: impl Fn(leptos::web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    let shown = display_score(score, &mode);
    let max   = max_score(&mode);
    let pct   = shown * 100 / max;
    let message = match pct {
        100     => "Perfect — flawless!",
        80..=99 => "Excellent work!",
        60..=79 => "Pretty good!",
        40..=59 => "Getting there.",
        _       => "Keep practising!",
    };
    view! {
        <div class="home">
            <h1 class="title">"Game over!"</h1>
            <p class="score">{shown}" / "{max}</p>
            <p class="subtitle">{message}</p>
            <button class="play-btn" on:click=on_play_again>"Play again"</button>
        </div>
    }
}
