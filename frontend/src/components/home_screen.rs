use common::types::GameMode;
use leptos::prelude::*;

use crate::daily::DailyEntry;
use crate::feedback_strings::FEEDBACK_BUTTON_LABEL;

#[component]
pub fn HomeScreen(
    on_play:      Callback<GameMode>,
    on_report:    Callback<()>,
    daily_result: Signal<Option<DailyEntry>>,
) -> impl IntoView {
    let show_picker = RwSignal::new(false);

    view! {
        <div class="home">
            <h1 class="title"><span class="title-accent">"Lingua"</span>"Guessr"</h1>
            <p class="subtitle">"Can you identify the language?"</p>
            <div class="primary-buttons">
                <div class="play-group">
                    <button
                        class="mode-btn play"
                        class:open=move || show_picker.get()
                        on:click=move |_| show_picker.update(|v| *v = !*v)
                    >
                        "PLAY"
                    </button>
                    {move || show_picker.get().then(|| view! {
                        <div class="difficulty-picker">
                            <button class="difficulty-btn easy"
                                on:click=move |_| {
                                    show_picker.set(false);
                                    on_play.run(GameMode::Easy);
                                }>
                                "Easy"
                            </button>
                            <button class="difficulty-btn medium"
                                on:click=move |_| {
                                    show_picker.set(false);
                                    on_play.run(GameMode::Medium);
                                }>
                                "Medium"
                            </button>
                            <button class="difficulty-btn hard"
                                on:click=move |_| {
                                    show_picker.set(false);
                                    on_play.run(GameMode::Hard);
                                }>
                                "Hard"
                            </button>
                        </div>
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
                                on:click=move |_| on_play.run(GameMode::Daily)>
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
