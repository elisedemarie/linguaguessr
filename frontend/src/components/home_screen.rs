use common::types::GameMode;
use leptos::prelude::*;

use crate::daily::DailyEntry;
use crate::feedback_strings::FEEDBACK_BUTTON_LABEL;
use crate::mode::mode_str;
use crate::seed::generate_seed;

#[component]
pub fn HomeScreen(
    on_start:     Callback<(GameMode, Option<String>)>,
    on_report:    Callback<()>,
    daily_result: Signal<Option<DailyEntry>>,
) -> impl IntoView {
    let show_picker      = RwSignal::new(false);
    let chal_open        = RwSignal::new(false);
    // When Some, a mode has been selected — (seed, mode). Seed is fixed for the session.
    let challenge_ready  = RwSignal::new(Option::<(String, GameMode)>::None);
    let link_copied      = RwSignal::new(false);

    let pick_challenge = move |mode: GameMode| {
        // Keep the same seed if already generated, so changing mode doesn't break shared links
        let seed = challenge_ready.get()
            .map(|(s, _)| s)
            .unwrap_or_else(generate_seed);
        link_copied.set(false);
        challenge_ready.set(Some((seed, mode)));
    };

    view! {
        <div class="home">
            <h1 class="title"><span class="title-accent">"Lingua"</span>"Guessr"</h1>
            <p class="subtitle">"Can you identify the language?"</p>
            <div class="primary-buttons">

                <div class="play-group">
                    <button
                        class="mode-btn play"
                        class:open=move || show_picker.get()
                        on:click=move |_| {
                            chal_open.set(false);
                            challenge_ready.set(None);
                            show_picker.update(|v| *v = !*v);
                        }
                    >
                        "PLAY"
                    </button>
                    {move || show_picker.get().then(|| view! {
                        <div class="difficulty-picker">
                            <button class="difficulty-btn easy"
                                on:click=move |_| {
                                    show_picker.set(false);
                                    on_start.run((GameMode::Easy, None));
                                }>
                                "Easy"
                            </button>
                            <button class="difficulty-btn medium"
                                on:click=move |_| {
                                    show_picker.set(false);
                                    on_start.run((GameMode::Medium, None));
                                }>
                                "Medium"
                            </button>
                            <button class="difficulty-btn hard"
                                on:click=move |_| {
                                    show_picker.set(false);
                                    on_start.run((GameMode::Hard, None));
                                }>
                                "Hard"
                            </button>
                        </div>
                    })}
                </div>

                <div class="play-group">
                    <button
                        class="mode-btn challenge"
                        class:open=move || chal_open.get()
                        on:click=move |_| {
                            show_picker.set(false);
                            if chal_open.get() {
                                chal_open.set(false);
                                challenge_ready.set(None);
                            } else {
                                chal_open.set(true);
                            }
                        }
                    >
                        "CHALLENGE"
                    </button>

                    {move || chal_open.get().then(|| {
                        let selected = challenge_ready.get().map(|(_, m)| m);
                        view! {
                            <div class="difficulty-picker">
                                <button
                                    class="difficulty-btn easy"
                                    class:selected=move || selected == Some(GameMode::Easy)
                                    on:click=move |_| pick_challenge(GameMode::Easy)>
                                    "Easy"
                                </button>
                                <button
                                    class="difficulty-btn medium"
                                    class:selected=move || selected == Some(GameMode::Medium)
                                    on:click=move |_| pick_challenge(GameMode::Medium)>
                                    "Medium"
                                </button>
                                <button
                                    class="difficulty-btn hard"
                                    class:selected=move || selected == Some(GameMode::Hard)
                                    on:click=move |_| pick_challenge(GameMode::Hard)>
                                    "Hard"
                                </button>
                            </div>
                        }
                    })}

                    {move || challenge_ready.get().map(|(seed, mode)| {
                        let origin = leptos::web_sys::window()
                            .and_then(|w| w.location().origin().ok())
                            .unwrap_or_else(|| "https://linguaguessr.io".into());
                        let share_url = format!("{}/?seed={}&mode={}", origin, seed, mode_str(&mode));
                        let share_url_copy = share_url.clone();

                        view! {
                            <div class="challenge-setup">
                                <p class="challenge-setup-label">"Share the link, then start when ready"</p>
                                <div class="challenge-setup-actions">
                                    <button class="copy-btn" on:click=move |_| {
                                        let url = share_url_copy.clone();
                                        link_copied.set(true);
                                        leptos::task::spawn_local(async move {
                                            if let Some(window) = leptos::web_sys::window() {
                                                let _ = wasm_bindgen_futures::JsFuture::from(
                                                    window.navigator().clipboard().write_text(&url)
                                                ).await;
                                            }
                                        });
                                    }>
                                        {move || if link_copied.get() { "Copied!" } else { "Copy link" }}
                                    </button>
                                    <button class="start-btn" on:click=move |_| {
                                        chal_open.set(false);
                                        challenge_ready.set(None);
                                        on_start.run((mode, Some(seed.clone())));
                                    }>
                                        "START"
                                    </button>
                                </div>
                            </div>
                        }
                    })}
                </div>

                <div class="daily-button">
                    {move || match daily_result.get() {
                        Some(entry) => view! {
                            <div class="daily-played">
                                <span class="daily-played-emojis">{entry.emojis}</span>
                                <span class="daily-played-score">{entry.score}" / 5000"</span>
                            </div>
                        }.into_any(),
                        None => view! {
                            <button class="mode-btn daily"
                                on:click=move |_| on_start.run((GameMode::Daily, None))>
                                "DAILY"
                            </button>
                        }.into_any(),
                    }}
                </div>

            </div>
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
