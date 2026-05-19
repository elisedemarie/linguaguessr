mod api;
mod components;
mod mode;
mod score;

use common::api::GameView;
use common::types::GameMode;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;

use api::fetch_game;
use components::{ErrorScreen, FinishedScreen, HomeScreen, LoadingScreen, RoundScreen};

#[derive(Clone)]
enum GamePhase {
    Home,
    Loading,
    Playing { game: GameView, mode: GameMode },
    Finished { score: u32, mode: GameMode },
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
                            mode=mode.clone()
                            on_finish=Callback::new(move |score| {
                                phase.set(GamePhase::Finished { score, mode: mode.clone() });
                            })
                        />
                    }.into_any()
                },
                GamePhase::Finished { score, mode } => view! {
                    <FinishedScreen score=score mode=mode on_play_again=go_home />
                }.into_any(),
                GamePhase::Error(msg) => view! {
                    <ErrorScreen message=msg />
                }.into_any(),
            }}
        </div>
    }
}
