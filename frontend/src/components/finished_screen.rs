use common::types::GameMode;
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::animation::ease_out_cubic;
use crate::feedback_strings::FEEDBACK_BUTTON_LABEL;
use crate::mode::{mode_str, show_score_breakdown};
use crate::score::{display_score, format_share_text, max_score, round_result_emoji};
use crate::RoundResult;

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    leptos::web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[component]
pub fn FinishedScreen(
    score:         u32,
    mode:          GameMode,
    rounds:        Vec<RoundResult>,
    seed:          Option<String>,
    on_play_again: impl Fn(leptos::web_sys::MouseEvent) + 'static,
    on_report:     Callback<()>,
) -> impl IntoView {
    let target     = display_score(score, &mode) as f64;
    let max        = max_score(&mode);
    let mode_label = mode.label().to_string();
    let breakdown  = show_score_breakdown(&mode);
    let is_daily   = mode == GameMode::Daily;

    let emojis: String = rounds.iter()
        .map(|r| round_result_emoji(r.response.score.total))
        .collect();

    let share_text = if is_daily {
        let d = js_sys::Date::new_0();
        let date_str = format!("{}-{:02}-{:02}",
            d.get_utc_full_year(),
            d.get_utc_month() + 1,
            d.get_utc_date());
        format_share_text(&date_str, &emojis, score)
    } else {
        String::new()
    };

    let copied            = RwSignal::new(false);
    let share_text_store  = StoredValue::new(share_text);
    let link_copied       = RwSignal::new(false);
    let is_seeded         = seed.is_some();
    let seed_store        = StoredValue::new(seed.unwrap_or_default());
    let mode_str_stored   = StoredValue::new(mode_str(&mode).to_string());

    let animated = RwSignal::new(0.0_f64);

    Effect::new(move |_| {
        let start: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(None));
        let cb: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let cb2 = cb.clone();

        *cb.borrow_mut() = Some(Closure::wrap(Box::new(move |ts: f64| {
            let mut s = start.borrow_mut();
            if s.is_none() {
                *s = Some(ts);
            }
            let elapsed = ts - s.unwrap();
            let t = (elapsed / 1200.0_f64).min(1.0);
            animated.set(ease_out_cubic(t) * target);

            if t < 1.0 {
                request_animation_frame(cb2.borrow().as_ref().unwrap());
            } else {
                animated.set(target);
                cb2.borrow_mut().take();
            }
        }) as Box<dyn FnMut(f64)>));

        request_animation_frame(cb.borrow().as_ref().unwrap());
    });

    let expanded = RwSignal::new(
        rounds.iter().map(|r| !r.response.correct).collect::<Vec<bool>>()
    );

    view! {
        <div class="home">
            <div class="end-mode-badge">{mode_label}</div>
            <p class="score">
                {move || animated.get() as u32}
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

            {is_daily.then(|| view! {
                <div class="daily-share">
                    <p class="daily-emojis">{emojis}</p>
                    <button class="copy-btn" on:click=move |_| {
                        let text = share_text_store.get_value();
                        copied.set(true);
                        leptos::task::spawn_local(async move {
                            if let Some(window) = leptos::web_sys::window() {
                                let clipboard = window.navigator().clipboard();
                                let _ = wasm_bindgen_futures::JsFuture::from(
                                    clipboard.write_text(&text)
                                ).await;
                            }
                        });
                    }>
                        {move || if copied.get() { "Copied!" } else { "Copy result" }}
                    </button>
                </div>
            })}

            {is_seeded.then(|| view! {
                <div class="seed-share">
                    <p class="seed-share-heading">"Challenge your friends"</p>
                    <p class="seed-share-sub">"Share this link — they'll play the exact same game"</p>
                    <p class="seed-code">{seed_store.get_value()}</p>
                    <button class="copy-btn" on:click=move |_| {
                        let seed = seed_store.get_value();
                        let mode = mode_str_stored.get_value();
                        link_copied.set(true);
                        leptos::task::spawn_local(async move {
                            if let Some(window) = leptos::web_sys::window() {
                                let origin = window.location().origin().unwrap_or_default();
                                let url = format!("{}/?seed={}&mode={}", origin, seed, mode);
                                let clipboard = window.navigator().clipboard();
                                let _ = wasm_bindgen_futures::JsFuture::from(
                                    clipboard.write_text(&url)
                                ).await;
                            }
                        });
                    }>
                        {move || if link_copied.get() { "Copied!" } else { "Copy link" }}
                    </button>
                </div>
            })}

            <button class="play-btn" on:click=on_play_again>"Play again"</button>

            <div class="end-actions">
                <a
                    href="https://github.com/elisedemarie/linguaguessr"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="github-link"
                    aria-label="GitHub repository"
                >
                    <svg viewBox="0 0 16 16" width="15" height="15" fill="currentColor" aria-hidden="true">
                        <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/>
                    </svg>
                </a>
                <button class="report-btn" on:click=move |_| on_report.run(())>
                    {FEEDBACK_BUTTON_LABEL}
                </button>
            </div>
        </div>
    }
}
