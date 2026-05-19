use common::types::GameMode;
use leptos::prelude::*;

#[component]
pub fn HomeScreen(on_play: Callback<GameMode>) -> impl IntoView {
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
