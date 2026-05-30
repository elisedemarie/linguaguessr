mod animation;
mod api;
mod components;
mod daily;
mod feedback_strings;
mod mode;
mod score;

use common::api::{GameView, GuessResponse};
use common::types::{GameMode, Language};
use leptos::prelude::*;
use leptos::task::spawn_local;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use api::fetch_game;
use components::{ErrorScreen, FeedbackModal, FinishedScreen, HomeScreen, LoadingScreen, RoundScreen};
use daily::{daily_already_played, DailyEntry, STORAGE_KEY};
use score::round_result_emoji;

#[derive(Clone)]
pub struct RoundResult {
    pub guessed:  Language,
    pub response: GuessResponse,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RoundContext {
    pub game_id:  Uuid,
    pub round_id: Uuid,
    pub language: Option<String>,
}

#[derive(Clone)]
enum GamePhase {
    Home,
    Loading,
    Playing { game: GameView, mode: GameMode },
    Finished { score: u32, mode: GameMode, rounds: Vec<RoundResult> },
    Error(String),
}

#[wasm_bindgen(start)]
pub fn main() {
    leptos::mount::mount_to_body(App);
}

fn today_utc() -> String {
    let d = js_sys::Date::new_0();
    format!("{}-{:02}-{:02}", d.get_utc_full_year(), d.get_utc_month() + 1, d.get_utc_date())
}

fn read_daily_entry() -> Option<DailyEntry> {
    let storage = leptos::web_sys::window()?.local_storage().ok()??;
    let json    = storage.get_item(STORAGE_KEY).ok()??;
    serde_json::from_str(&json).ok()
}

fn write_daily_entry(entry: &DailyEntry) {
    if let Some(Ok(Some(storage))) = leptos::web_sys::window()
        .map(|w| w.local_storage())
    {
        if let Ok(json) = serde_json::to_string(entry) {
            let _ = storage.set_item(STORAGE_KEY, &json);
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let phase         = RwSignal::new(GamePhase::Home);
    let modal_open    = RwSignal::new(false);
    let round_context = RwSignal::new(Option::<RoundContext>::None);
    let daily_result  = RwSignal::new(Option::<DailyEntry>::None);

    Effect::new(move |_| {
        let today = today_utc();
        if let Some(entry) = read_daily_entry() {
            if daily_already_played(&entry, &today) {
                daily_result.set(Some(entry));
            }
        }
    });

    let go_home = move |_| {
        round_context.set(None);
        phase.set(GamePhase::Home);
    };

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
                    <HomeScreen
                        on_play=start_game
                        on_report=Callback::new(move |()| modal_open.set(true))
                        daily_result=Signal::derive(move || daily_result.get())
                    />
                }.into_any(),
                GamePhase::Loading => view! {
                    <LoadingScreen />
                }.into_any(),
                GamePhase::Playing { game, mode } => {
                    view! {
                        <RoundScreen
                            game=game
                            mode=mode.clone()
                            round_context=round_context
                            on_finish=Callback::new(move |(score, rounds): (u32, Vec<RoundResult>)| {
                                if mode == GameMode::Daily {
                                    let emojis: String = rounds.iter()
                                        .map(|r| round_result_emoji(r.response.score.total))
                                        .collect();
                                    let entry = DailyEntry { date: today_utc(), emojis, score };
                                    write_daily_entry(&entry);
                                    daily_result.set(Some(entry));
                                }
                                phase.set(GamePhase::Finished { score, mode: mode.clone(), rounds });
                            })
                        />
                    }.into_any()
                },
                GamePhase::Finished { score, mode, rounds } => view! {
                    <FinishedScreen
                        score=score
                        mode=mode
                        rounds=rounds
                        on_play_again=go_home
                        on_report=Callback::new(move |()| modal_open.set(true))
                    />
                }.into_any(),
                GamePhase::Error(msg) => view! {
                    <ErrorScreen message=msg />
                }.into_any(),
            }}
            <FeedbackModal open=modal_open context=round_context />
        </div>
    }
}
