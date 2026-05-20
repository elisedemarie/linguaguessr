use common::types::GameMode;
use leptos::prelude::*;

use crate::mode::show_score_breakdown;
use crate::score::{display_score, max_score};
use crate::RoundResult;

#[component]
pub fn FinishedScreen(
    score: u32,
    mode: GameMode,
    rounds: Vec<RoundResult>,
    on_play_again: impl Fn(leptos::web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    let shown = display_score(score, &mode);
    let max   = max_score(&mode);
    let mode_label = mode.label().to_string();
    let breakdown = show_score_breakdown(&mode);

    view! {
        <div class="home">
            <div class="end-mode-badge">{mode_label}</div>
            <h1 class="title">"Game over!"</h1>
            <p class="score">
                {shown}
                {(mode == GameMode::Easy).then(|| format!(" / {max}"))}
            </p>


            <div class="end-breakdown">
                {rounds.into_iter().enumerate().map(|(i, r)| {
                    let correct = r.response.correct;
                    let correct_lang = r.response.correct_language.label().to_string();
                    let guessed_lang = r.guessed.label().to_string();
                    let round_score  = r.response.score.total;
                    let row_class    = if correct { "end-round correct" } else { "end-round wrong" };
                    let tick         = if correct { "✓" } else { "✗" };

                    view! {
                        <div class=row_class>
                            <span class="end-round-num">{i + 1}</span>
                            <span class="end-round-tick">{tick}</span>
                            <span class="end-round-lang">
                                {correct_lang}
                                {(!correct).then(|| view! {
                                    <span class="end-round-guessed">" — you said "{guessed_lang}</span>
                                })}
                            </span>
                            {breakdown.then(|| view! {
                                <span class="end-round-score">{round_score}</span>
                            })}
                        </div>
                    }
                }).collect_view()}
            </div>

            <button class="play-btn" on:click=on_play_again>"Play again"</button>
        </div>
    }
}

