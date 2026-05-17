use common::types::Language;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let selected = RwSignal::new(Option::<Language>::None);

    view! {
        <div>
            <LanguageCombobox on_select=Callback::new(move |lang| selected.set(Some(lang))) />
            {move || selected.get().map(|lang| view! {
                <p style="margin-top: 1rem">
                    "Selected: " {lang.label().to_string()}
                </p>
            })}
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
