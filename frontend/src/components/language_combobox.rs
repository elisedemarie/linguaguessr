use common::types::Language;
use leptos::prelude::*;

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
