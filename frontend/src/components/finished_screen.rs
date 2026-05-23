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

    let expanded = RwSignal::new(
        rounds.iter().map(|r| !r.response.correct).collect::<Vec<bool>>()
    );

    view! {
        <div class="home">
            <div class="end-mode-badge">{mode_label}</div>
            <p class="score">
                {shown}
                {(mode == GameMode::Easy).then(|| format!(" / {max}"))}
            </p>

            <div class="end-breakdown">
                {rounds.into_iter().enumerate().map(|(i, r)| {
                    let correct      = r.response.correct;
                    let correct_lang = r.response.correct_language.label().to_string();
                    let guessed_lang = r.guessed.label().to_string();
                    let round_score  = r.response.score.total;
                    let script_score = r.response.score.script;
                    let family_score = r.response.score.family;
                    let script_label = StoredValue::new(r.response.labels.script.clone());
                    let family_label = StoredValue::new(r.response.labels.family.clone());
                    let row_class    = if correct { "end-round correct" } else { "end-round wrong" };
                    let tick         = if correct { "✓" } else { "✗" };

                    view! {
                        <div class=row_class>
                            <div
                                class="end-round-summary"
                                class:clickable=breakdown
                                on:click=move |_| {
                                    if breakdown {
                                        expanded.update(|v| v[i] = !v[i]);
                                    }
                                }
                            >
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
                                    <span class="end-round-chevron">
                                        {move || if expanded.get()[i] { "▲" } else { "▼" }}
                                    </span>
                                })}
                            </div>

                            {breakdown.then(move || view! {
                                <Show when=move || expanded.get()[i]>
                                    <div class="end-round-detail">
                                        <div class="end-detail-axis">
                                            <span class="end-detail-name">"Script"</span>
                                            <div class="axis-bar">
                                                <div class="axis-fill"
                                                    style=format!("width: {}%", script_score / 5)>
                                                </div>
                                            </div>
                                            <span class="end-detail-label">{script_label.get_value()}</span>
                                        </div>
                                        <div class="end-detail-axis">
                                            <span class="end-detail-name">"Family"</span>
                                            <div class="axis-bar">
                                                <div class="axis-fill"
                                                    style=format!("width: {}%", family_score / 5)>
                                                </div>
                                            </div>
                                            <span class="end-detail-label">{family_label.get_value()}</span>
                                        </div>
                                    </div>
                                </Show>
                            })}
                        </div>
                    }
                }).collect_view()}
            </div>

            <button class="play-btn" on:click=on_play_again>"Play again"</button>
        </div>
    }
}
