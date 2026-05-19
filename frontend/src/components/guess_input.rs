use common::types::Language;
use leptos::prelude::*;

use super::language_combobox::LanguageCombobox;

#[component]
pub fn GuessInput(
    options: Vec<Language>,
    pool: &'static [Language],
    query: RwSignal<String>,
    selected: RwSignal<Option<Language>>,
    submitting: RwSignal<bool>,
    on_submit: Callback<leptos::web_sys::MouseEvent>,
) -> impl IntoView {
    if options.is_empty() {
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
                    on:click=move |ev| on_submit.run(ev)
                >
                    {move || if submitting.get() { "Submitting..." } else { "Submit" }}
                </button>
            </div>
        }.into_any()
    } else {
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
                    on:click=move |ev| on_submit.run(ev)
                >
                    {move || if submitting.get() { "Submitting..." } else { "Submit" }}
                </button>
            </div>
        }.into_any()
    }
}
