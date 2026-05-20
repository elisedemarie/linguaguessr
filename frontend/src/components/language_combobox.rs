use common::types::Language;
use leptos::prelude::*;

pub fn next_highlighted(current: Option<usize>, count: usize) -> Option<usize> {
    match current {
        _ if count == 0 => None,
        None    => Some(0),
        Some(i) => Some((i + 1).min(count - 1)),
    }
}

pub fn prev_highlighted(current: Option<usize>) -> Option<usize> {
    match current {
        None | Some(0) => None,
        Some(i)        => Some(i - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- next_highlighted ---

    #[test]
    fn next_highlighted_none_with_no_items_returns_none() {
        assert_eq!(next_highlighted(None, 0), None);
    }

    #[test]
    fn next_highlighted_none_with_items_returns_first() {
        assert_eq!(next_highlighted(None, 3), Some(0));
    }

    #[test]
    fn next_highlighted_advances_index() {
        assert_eq!(next_highlighted(Some(0), 3), Some(1));
    }

    #[test]
    fn next_highlighted_clamps_at_last_item() {
        assert_eq!(next_highlighted(Some(2), 3), Some(2));
    }

    #[test]
    fn next_highlighted_single_item_stays_at_zero() {
        assert_eq!(next_highlighted(Some(0), 1), Some(0));
    }

    // --- prev_highlighted ---

    #[test]
    fn prev_highlighted_none_returns_none() {
        assert_eq!(prev_highlighted(None), None);
    }

    #[test]
    fn prev_highlighted_first_item_returns_none() {
        assert_eq!(prev_highlighted(Some(0)), None);
    }

    #[test]
    fn prev_highlighted_decrements_index() {
        assert_eq!(prev_highlighted(Some(2)), Some(1));
    }

    #[test]
    fn prev_highlighted_from_one_returns_zero() {
        assert_eq!(prev_highlighted(Some(1)), Some(0));
    }
}

#[component]
pub fn LanguageCombobox(
    on_select: Callback<Language>,
    query: RwSignal<String>,
    pool: &'static [Language],
    #[prop(optional)] on_input_change: Option<Callback<()>>,
) -> impl IntoView {
    let is_open    = RwSignal::new(false);
    let highlighted = RwSignal::new(Option::<usize>::None);
    let suggestions = Memo::new(move |_| Language::suggestions_for(&query.get(), pool));

    let select = Callback::new(move |lang: Language| {
        let label = lang.label().to_string();
        on_select.run(lang);
        query.set(label);
        is_open.set(false);
        highlighted.set(None);
    });

    view! {
        <div class="combobox">
            <input
                type="text"
                placeholder="Type a language..."
                autocomplete="off"
                autocapitalize="none"
                prop:value=move || query.get()
                on:input=move |ev| {
                    query.set(event_target_value(&ev));
                    is_open.set(true);
                    highlighted.set(None);
                    if let Some(cb) = on_input_change { cb.run(()); }
                }
                on:focus=move |_| is_open.set(true)
                on:keydown=move |ev: leptos::web_sys::KeyboardEvent| {
                    let items = suggestions.get();
                    match ev.key().as_str() {
                        "ArrowDown" => {
                            ev.prevent_default();
                            is_open.set(true);
                            highlighted.update(|h| *h = next_highlighted(*h, items.len()));
                        }
                        "ArrowUp" => {
                            ev.prevent_default();
                            highlighted.update(|h| *h = prev_highlighted(*h));
                        }
                        "Enter" => {
                            ev.prevent_default();
                            if let Some(i) = highlighted.get() {
                                if let Some(lang) = items.get(i) {
                                    select.run(lang.clone());
                                }
                            } else if items.len() == 1 {
                                select.run(items[0].clone());
                            }
                        }
                        "Escape" => {
                            is_open.set(false);
                            highlighted.set(None);
                        }
                        _ => {}
                    }
                }
            />
            <Show when=move || is_open.get() && !suggestions.get().is_empty()>
                <ul class="suggestions">
                    {move || suggestions.get().into_iter().enumerate().map(|(i, lang)| {
                        let label = lang.label().to_string();
                        view! {
                            <li
                                class:highlighted=move || highlighted.get() == Some(i)
                                on:click=move |_| select.run(lang.clone())
                            >
                                {label}
                            </li>
                        }
                    }).collect_view()}
                </ul>
            </Show>
        </div>
    }
}
