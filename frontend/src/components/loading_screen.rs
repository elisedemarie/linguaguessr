use leptos::prelude::*;

#[component]
pub fn LoadingScreen() -> impl IntoView {
    view! {
        <div class="loading"><p>"Loading your game..."</p></div>
    }
}
