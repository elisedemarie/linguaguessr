use common::api::GuessResponse;
use leptos::prelude::*;

fn script_tooltip_text() -> &'static str {
    "How similar was the script of your guess to the answer?"
}

fn family_tooltip_text() -> &'static str {
    "How closely related was the language of your guess to the answer?"
}

#[component]
pub fn FeedbackPanel(
    feedback: GuessResponse,
    breakdown: bool,
    round_index: RwSignal<usize>,
    total: usize,
    on_next: Callback<leptos::web_sys::MouseEvent>,
) -> impl IntoView {
    let header = if feedback.correct {
        "✓ Correct!".to_string()
    } else {
        format!("✗ Wrong — it was {}", feedback.correct_language.label())
    };
    let header_class = if feedback.correct { "feedback correct" } else { "feedback wrong" };
    let script_score = feedback.score.script;
    let family_score = feedback.score.family;
    let total_score  = feedback.score.total;
    let script_label = feedback.labels.script.clone();
    let family_label = feedback.labels.family.clone();

    if breakdown {
        view! {
            <div class=header_class>
                <p class="feedback-header">{header}</p>
                <div class="score-axis">
                    <span class="axis-name">
                        "Script"
                        <span class="tooltip">{script_tooltip_text()}</span>
                    </span>
                    <div class="axis-bar">
                        <div class="axis-fill"
                            style=format!("width: {}%", script_score / 5)>
                        </div>
                    </div>
                    <span class="axis-score">{script_score}" / 500"</span>
                    {(total_score < 1000).then(|| view! { <span class="axis-desc">{script_label}</span> })}
                </div>
                <div class="score-axis">
                    <span class="axis-name">
                        "Family"
                        <span class="tooltip">{family_tooltip_text()}</span>
                    </span>
                    <div class="axis-bar">
                        <div class="axis-fill"
                            style=format!("width: {}%", family_score / 5)>
                        </div>
                    </div>
                    <span class="axis-score">{family_score}" / 500"</span>
                    {(total_score < 1000).then(|| view! { <span class="axis-desc">{family_label}</span> })}
                </div>
                <p class="score-total">{total_score}" / 1,000"</p>
            </div>
            <button class="next-btn" on:click=move |ev| on_next.run(ev)>
                {move || if round_index.get() + 1 >= total { "See my score" } else { "Next →" }}
            </button>
        }.into_any()
    } else {
        view! {
            <div class=header_class>
                <p class="feedback-header">{header}</p>
            </div>
            <button class="next-btn" on:click=move |ev| on_next.run(ev)>
                {move || if round_index.get() + 1 >= total { "See my score" } else { "Next →" }}
            </button>
        }.into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_tooltip_text_is_correct() {
        assert_eq!(script_tooltip_text(), "How similar was the script of your guess to the answer?");
    }

    #[test]
    fn family_tooltip_text_is_correct() {
        assert_eq!(family_tooltip_text(), "How closely related was the language of your guess to the answer?");
    }
}
