use common::api::{GameView, GuessRequest, GuessResponse};
use common::types::Language;
use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

const BACKEND_URL: &str = "http://localhost:3000";

#[derive(Clone)]
enum GamePhase {
    Home,
    Loading,
    Playing(GameView),
    Error(String),
}

#[wasm_bindgen(start)]
pub fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let phase = RwSignal::new(GamePhase::Home);

    let start_game = move |_| {
        phase.set(GamePhase::Loading);
        spawn_local(async move {
            match fetch_game().await {
                Ok(game) => phase.set(GamePhase::Playing(game)),
                Err(e) => phase.set(GamePhase::Error(e)),
            }
        });
    };

    view! {
        <div class="app">
            {move || match phase.get() {
                GamePhase::Home => view! { <HomeScreen on_play=start_game /> }.into_any(),
                GamePhase::Loading => view! { <LoadingScreen /> }.into_any(),
                GamePhase::Playing(game) => view! { <RoundScreen game=game /> }.into_any(),
                GamePhase::Error(msg) => view! { <ErrorScreen message=msg /> }.into_any(),
            }}
        </div>
    }
}

async fn fetch_game() -> Result<GameView, String> {
    let response = gloo_net::http::Request::get(&format!("{BACKEND_URL}/api/game"))
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

#[component]
fn HomeScreen(on_play: impl Fn(leptos::web_sys::MouseEvent) + 'static) -> impl IntoView {
    view! {
        <div class="home">
            <h1 class="title">"LinguaGuessr"</h1>
            <p class="subtitle">"Can you identify the language?"</p>
            <button class="play-btn" on:click=on_play>"Play"</button>
        </div>
    }
}

#[component]
fn LoadingScreen() -> impl IntoView {
    view! {
        <div class="loading">
            <p>"Loading your game..."</p>
        </div>
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
fn RoundScreen(game: GameView) -> impl IntoView {
    let game_id = game.game_id;
    let total = game.rounds.len();
    let round_index = RwSignal::new(0usize);
    let rounds = StoredValue::new(game.rounds);

    let current_round = Memo::new(move |_| {
        rounds.with_value(|r| r[round_index.get()].clone())
    });

    let selected = RwSignal::new(Option::<Language>::None);
    let feedback = RwSignal::new(Option::<GuessResponse>::None);
    let submitting = RwSignal::new(false);

    let on_submit = move |_| {
        let Some(lang) = selected.get() else { return };
        let round_id = current_round.get().round_id;
        submitting.set(true);
        spawn_local(async move {
            match submit_guess(game_id, round_id, lang).await {
                Ok(result) => {
                    feedback.set(Some(result));
                    submitting.set(false);
                }
                Err(_) => submitting.set(false),
            }
        });
    };

    view! {
        <div class="round">
            <p class="round-counter">
                "Round " {move || round_index.get() + 1} " of " {total}
            </p>
            <div class="text-block">
                {move || current_round.get().text}
            </div>

            // Input area — hidden once feedback arrives
            <Show when=move || feedback.get().is_none()>
                <LanguageCombobox on_select=Callback::new(move |lang| {
                    selected.set(Some(lang));
                }) />
                <button
                    class="submit-btn"
                    prop:disabled=move || selected.get().is_none() || submitting.get()
                    on:click=on_submit
                >
                    {move || if submitting.get() { "Submitting..." } else { "Submit" }}
                </button>
            </Show>

            // Feedback — shown after submission
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
            </Show>
        </div>
    }
}

#[component]
pub fn LanguageCombobox(on_select: Callback<Language>) -> impl IntoView {
    let query = RwSignal::new(String::new());
    let is_open = RwSignal::new(false);
    let suggestions = Memo::new(move |_| Language::suggestions(&query.get()));

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
