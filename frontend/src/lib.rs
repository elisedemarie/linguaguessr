use common::api::{GameView, GuessRequest, GuessResponse};
use common::types::{GameMode, Language};
use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

const BACKEND_URL: &str = match option_env!("BACKEND_URL") {
    Some(url) => url,
    None => "http://localhost:3000",
};

#[derive(Clone)]
enum GamePhase {
    Home,
    Loading,
    Playing { game: GameView, mode: GameMode },
    Finished { score: u32 },
    Error(String),
}

#[wasm_bindgen(start)]
pub fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let phase = RwSignal::new(GamePhase::Home);

    let go_home = move |_| phase.set(GamePhase::Home);

    let start_game = Callback::new(move |mode: GameMode| {
        phase.set(GamePhase::Loading);
        spawn_local(async move {
            match fetch_game(&mode).await {
                Ok(game) => phase.set(GamePhase::Playing { game, mode }),
                Err(e)   => phase.set(GamePhase::Error(e)),
            }
        });
    });

    view! {
        <div class="app">
            {move || match phase.get() {
                GamePhase::Home => view! {
                    <HomeScreen on_play=start_game />
                }.into_any(),
                GamePhase::Loading => view! {
                    <LoadingScreen />
                }.into_any(),
                GamePhase::Playing { game, mode } => {
                    view! {
                        <RoundScreen
                            game=game
                            mode=mode
                            on_finish=Callback::new(move |score| {
                                phase.set(GamePhase::Finished { score });
                            })
                        />
                    }.into_any()
                },
                GamePhase::Finished { score } => view! {
                    <FinishedScreen score=score on_play_again=go_home />
                }.into_any(),
                GamePhase::Error(msg) => view! {
                    <ErrorScreen message=msg />
                }.into_any(),
            }}
        </div>
    }
}

fn mode_str(mode: &GameMode) -> &'static str {
    match mode {
        GameMode::Easy   => "easy",
        GameMode::Medium => "medium",
        GameMode::Hard   => "hard",
    }
}

async fn fetch_game(mode: &GameMode) -> Result<GameView, String> {
    let response = gloo_net::http::Request::get(
        &format!("{BACKEND_URL}/api/game?mode={}", mode_str(mode))
    )
    .send()
    .await
    .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response.json::<GameView>().await.map_err(|e| format!("Parse error: {e}"))
}

async fn submit_guess(
    game_id: Uuid,
    round_id: Uuid,
    language: Language,
) -> Result<GuessResponse, String> {
    let body = serde_json::to_string(&GuessRequest { round_id, language })
        .map_err(|e| format!("Serialise error: {e}"))?;

    let response = gloo_net::http::Request::post(
        &format!("{BACKEND_URL}/api/game/{game_id}/guess"),
    )
    .header("Content-Type", "application/json")
    .body(body)
    .map_err(|e| format!("Request error: {e}"))?
    .send()
    .await
    .map_err(|e| format!("Network error: {e}"))?;

    if !response.ok() {
        return Err(format!("Server error: {}", response.status()));
    }

    response.json::<GuessResponse>().await.map_err(|e| format!("Parse error: {e}"))
}

fn script_tooltip_text() -> &'static str {
    "How similar was the script of your guess to the answer?"
}

fn family_tooltip_text() -> &'static str {
    "How closely related was the language of your guess to the answer?"
}

fn suggestion_pool(mode: &GameMode) -> &'static [Language] {
    match mode {
        GameMode::Easy   => Language::easy_pool(),
        GameMode::Medium => Language::medium_pool(),
        GameMode::Hard   => Language::all(),
    }
}

#[component]
fn HomeScreen(on_play: Callback<GameMode>) -> impl IntoView {
    view! {
        <div class="home">
            <h1 class="title">"LinguaGuessr"</h1>
            <p class="subtitle">"Can you identify the language?"</p>
            <div class="mode-buttons">
                <button class="mode-btn easy"
                    on:click=move |_| on_play.run(GameMode::Easy)>
                    "Easy"
                </button>
                <button class="mode-btn medium"
                    on:click=move |_| on_play.run(GameMode::Medium)>
                    "Medium"
                </button>
                <button class="mode-btn hard"
                    on:click=move |_| on_play.run(GameMode::Hard)>
                    "Hard"
                </button>
            </div>
        </div>
    }
}

#[component]
fn LoadingScreen() -> impl IntoView {
    view! {
        <div class="loading"><p>"Loading your game..."</p></div>
    }
}

#[component]
fn ErrorScreen(message: String) -> impl IntoView {
    view! {
        <div class="error">
            <p>"Something went wrong."</p>
            <p class="error-detail">{message}</p>
        </div>
    }
}

#[component]
fn FinishedScreen(
    score: u32,
    on_play_again: impl Fn(leptos::web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    let pct = score * 100 / 5000;
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
            <p class="score">{score}" / 5,000"</p>
            <p class="subtitle">{message}</p>
            <button class="play-btn" on:click=on_play_again>"Play again"</button>
        </div>
    }
}

#[component]
fn RoundScreen(game: GameView, mode: GameMode, on_finish: Callback<u32>) -> impl IntoView {
    let game_id   = game.game_id;
    let total     = game.rounds.len();
    let rounds    = StoredValue::new(game.rounds);
    let pool      = suggestion_pool(&mode);

    let round_index = RwSignal::new(0usize);
    let score       = RwSignal::new(0u32);
    let selected    = RwSignal::new(Option::<Language>::None);
    let feedback    = RwSignal::new(Option::<GuessResponse>::None);
    let submitting  = RwSignal::new(false);
    let query       = RwSignal::new(String::new());

    let current_round = Memo::new(move |_| {
        rounds.with_value(|r| r[round_index.get()].clone())
    });

    let on_submit = move |_| {
        let Some(lang) = selected.get() else { return };
        let round_id = current_round.get().round_id;
        submitting.set(true);
        spawn_local(async move {
            match submit_guess(game_id, round_id, lang).await {
                Ok(result) => {
                    score.update(|s| *s += result.score.total);
                    feedback.set(Some(result));
                    submitting.set(false);
                }
                Err(_) => submitting.set(false),
            }
        });
    };

    let on_next = move |_| {
        let next = round_index.get() + 1;
        if next >= total {
            on_finish.run(score.get());
        } else {
            round_index.set(next);
            selected.set(None);
            feedback.set(None);
            query.set(String::new());
        }
    };

    view! {
        <div class="round">
            <p class="round-counter">
                "Round " {move || round_index.get() + 1} " of " {total}
            </p>
            <div class="text-block">
                {move || current_round.get().text}
            </div>

            <Show when=move || feedback.get().is_none()>
                {move || {
                    let round = current_round.get();
                    if round.options.is_empty() {
                        view! {
                            <div class="combobox-group">
                                <LanguageCombobox
                                    query=query
                                    pool=pool
                                    on_select=Callback::new(move |lang| selected.set(Some(lang)))
                                />
                                <button
                                    class="submit-btn"
                                    prop:disabled=move || selected.get().is_none() || submitting.get()
                                    on:click=on_submit
                                >
                                    {move || if submitting.get() { "Submitting..." } else { "Submit" }}
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        let options = round.options.clone();
                        view! {
                            <div class="option-buttons">
                                {options.into_iter().map(|lang| {
                                    let label = lang.label().to_string();
                                    let lang_for_click = lang.clone();
                                    let is_selected = move || selected.get().as_ref() == Some(&lang);
                                    view! {
                                        <button
                                            class="option-btn"
                                            class:selected=is_selected
                                            on:click=move |_| selected.set(Some(lang_for_click.clone()))
                                        >
                                            {label}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                                <button
                                    class="submit-btn"
                                    prop:disabled=move || selected.get().is_none() || submitting.get()
                                    on:click=on_submit
                                >
                                    {move || if submitting.get() { "Submitting..." } else { "Submit" }}
                                </button>
                            </div>
                        }.into_any()
                    }
                }}
            </Show>

            <Show when=move || feedback.get().is_some()>
                {move || feedback.get().map(|f| {
                    let header = if f.correct {
                        "✓ Correct!".to_string()
                    } else {
                        format!("✗ Wrong — it was {}", f.correct_language.label())
                    };
                    let header_class = if f.correct { "feedback correct" } else { "feedback wrong" };
                    let script_score  = f.score.script;
                    let family_score  = f.score.family;
                    let total_score   = f.score.total;
                    let script_label  = f.labels.script.clone();
                    let family_label  = f.labels.family.clone();
                    view! {
                        <div class=header_class>
                            <p class="feedback-header">{header}</p>
                            <div class="score-axis">
                                <span class="axis-name">
                                    "Script"
                                    <span class="info-icon">
                                        "ⓘ"
                                        <span class="tooltip">{script_tooltip_text()}</span>
                                    </span>
                                </span>
                                <div class="axis-bar">
                                    <div class="axis-fill"
                                        style=format!("width: {}%", script_score / 5)>
                                    </div>
                                </div>
                                <span class="axis-score">{script_score}" / 500"</span>
                                <span class="axis-desc">{script_label}</span>
                            </div>
                            <div class="score-axis">
                                <span class="axis-name">
                                    "Family"
                                    <span class="info-icon">
                                        "ⓘ"
                                        <span class="tooltip">{family_tooltip_text()}</span>
                                    </span>
                                </span>
                                <div class="axis-bar">
                                    <div class="axis-fill"
                                        style=format!("width: {}%", family_score / 5)>
                                    </div>
                                </div>
                                <span class="axis-score">{family_score}" / 500"</span>
                                <span class="axis-desc">{family_label}</span>
                            </div>
                            <p class="score-total">{total_score}" / 1,000"</p>
                        </div>
                    }.into_any()
                })}
                <button class="next-btn" on:click=on_next>
                    {move || if round_index.get() + 1 >= total { "See my score" } else { "Next →" }}
                </button>
            </Show>
        </div>
    }
}

#[component]
pub fn LanguageCombobox(
    on_select: Callback<Language>,
    query: RwSignal<String>,
    pool: &'static [Language],
) -> impl IntoView {
    let is_open     = RwSignal::new(false);
    let suggestions = Memo::new(move |_| Language::suggestions_for(&query.get(), pool));

    view! {
        <div class="combobox">
            <input
                type="text"
                placeholder="Type a language..."
                prop:value=move || query.get()
                on:input=move |ev| {
                    query.set(event_target_value(&ev));
                    is_open.set(true);
                }
                on:focus=move |_| is_open.set(true)
            />
            <Show when=move || is_open.get() && !suggestions.get().is_empty()>
                <ul class="suggestions">
                    <For
                        each=move || suggestions.get()
                        key=|lang| lang.label().to_string()
                        children=move |lang| {
                            let label = lang.label().to_string();
                            let label_display = label.clone();
                            view! {
                                <li on:click=move |_| {
                                    on_select.run(lang.clone());
                                    query.set(label.clone());
                                    is_open.set(false);
                                }>
                                    {label_display}
                                </li>
                            }
                        }
                    />
                </ul>
            </Show>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- tooltip copy ---

    #[test]
    fn script_tooltip_text_is_correct() {
        assert_eq!(script_tooltip_text(), "How similar was the script of your guess to the answer?");
    }

    #[test]
    fn family_tooltip_text_is_correct() {
        assert_eq!(family_tooltip_text(), "How closely related was the language of your guess to the answer?");
    }

    // --- mode_str ---

    #[test]
    fn mode_str_easy() { assert_eq!(mode_str(&GameMode::Easy), "easy"); }
    #[test]
    fn mode_str_medium() { assert_eq!(mode_str(&GameMode::Medium), "medium"); }
    #[test]
    fn mode_str_hard() { assert_eq!(mode_str(&GameMode::Hard), "hard"); }

    // --- suggestion_pool ---

    #[test]
    fn suggestion_pool_easy_returns_10() {
        assert_eq!(suggestion_pool(&GameMode::Easy).len(), 10);
    }
    #[test]
    fn suggestion_pool_medium_returns_30() {
        assert_eq!(suggestion_pool(&GameMode::Medium).len(), 30);
    }
    #[test]
    fn suggestion_pool_hard_returns_75() {
        assert_eq!(suggestion_pool(&GameMode::Hard).len(), 75);
    }
}
