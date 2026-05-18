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
    Finished { score: usize, total: usize },
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
                    let total = game.rounds.len();
                    view! {
                        <RoundScreen
                            game=game
                            mode=mode
                            on_finish=Callback::new(move |score| {
                                phase.set(GamePhase::Finished { score, total });
                            })
                        />
                    }.into_any()
                },
                GamePhase::Finished { score, total } => view! {
                    <FinishedScreen score=score total=total on_play_again=go_home />
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
    score: usize,
    total: usize,
    on_play_again: impl Fn(leptos::web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <div class="home">
            <h1 class="title">"Game over!"</h1>
            <p class="score">{score}" / "{total}</p>
            <p class="subtitle">{score_message(score, total)}</p>
            <button class="play-btn" on:click=on_play_again>"Play again"</button>
        </div>
    }
}

fn score_message(score: usize, total: usize) -> &'static str {
    match score * 100 / total {
        100          => "Perfect — flawless!",
        80..=99      => "Excellent work!",
        60..=79      => "Pretty good!",
        40..=59      => "Getting there.",
        _            => "Keep practising!",
    }
}

#[component]
fn RoundScreen(game: GameView, mode: GameMode, on_finish: Callback<usize>) -> impl IntoView {
    let game_id   = game.game_id;
    let total     = game.rounds.len();
    let rounds    = StoredValue::new(game.rounds);
    let pool      = suggestion_pool(&mode);

    let round_index = RwSignal::new(0usize);
    let score       = RwSignal::new(0usize);
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
                    if result.correct { score.update(|s| *s += 1); }
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
                            <div>
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
                    if f.correct {
                        view! {
                            <div class="feedback correct">"✓ Correct!"</div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="feedback wrong">
                                "✗ Wrong — it was "
                                <strong>{f.correct_language.label().to_string()}</strong>
                            </div>
                        }.into_any()
                    }
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

    // --- realistic game inputs (score 0–5 out of 5) ---

    #[test]
    fn zero_out_of_five_is_keep_practising() {
        assert_eq!(score_message(0, 5), "Keep practising!");
    }

    #[test]
    fn one_out_of_five_is_keep_practising() {
        assert_eq!(score_message(1, 5), "Keep practising!");
    }

    #[test]
    fn two_out_of_five_is_getting_there() {
        assert_eq!(score_message(2, 5), "Getting there.");
    }

    #[test]
    fn three_out_of_five_is_pretty_good() {
        assert_eq!(score_message(3, 5), "Pretty good!");
    }

    #[test]
    fn four_out_of_five_is_excellent() {
        assert_eq!(score_message(4, 5), "Excellent work!");
    }

    #[test]
    fn five_out_of_five_is_perfect() {
        assert_eq!(score_message(5, 5), "Perfect — flawless!");
    }

    // --- boundary values for each range ---

    #[test]
    fn exactly_100_percent_is_perfect() {
        assert_eq!(score_message(100, 100), "Perfect — flawless!");
    }

    #[test]
    fn exactly_99_percent_is_excellent() {
        assert_eq!(score_message(99, 100), "Excellent work!");
    }

    #[test]
    fn exactly_80_percent_is_excellent() {
        assert_eq!(score_message(80, 100), "Excellent work!");
    }

    #[test]
    fn exactly_79_percent_is_pretty_good() {
        assert_eq!(score_message(79, 100), "Pretty good!");
    }

    #[test]
    fn exactly_60_percent_is_pretty_good() {
        assert_eq!(score_message(60, 100), "Pretty good!");
    }

    #[test]
    fn exactly_59_percent_is_getting_there() {
        assert_eq!(score_message(59, 100), "Getting there.");
    }

    #[test]
    fn exactly_40_percent_is_getting_there() {
        assert_eq!(score_message(40, 100), "Getting there.");
    }

    #[test]
    fn exactly_39_percent_is_keep_practising() {
        assert_eq!(score_message(39, 100), "Keep practising!");
    }

    #[test]
    fn zero_percent_is_keep_practising() {
        assert_eq!(score_message(0, 100), "Keep practising!");
    }
}
