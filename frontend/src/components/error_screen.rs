use leptos::prelude::*;

#[component]
pub fn ErrorScreen(message: String) -> impl IntoView {
    view! {
        <div class="error">
            <p>"Something went wrong."</p>
            <p class="error-detail">{message}</p>
        </div>
    }
}
