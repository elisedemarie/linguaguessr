use common::api::{GameView, GuessResponse};
use common::types::Language;
use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::api::submit_guess;
use crate::mode::{show_score_breakdown, suggestion_pool};
use super::feedback_panel::FeedbackPanel;
use super::guess_input::GuessInput;

#[component]
pub fn RoundScreen(game: GameView, mode: common::types::GameMode, on_finish: Callback<u32>) -> impl IntoView {
    let game_id   = game.game_id;
    let total     = game.rounds.len();
    let rounds    = StoredValue::new(game.rounds);
    let pool      = suggestion_pool(&mode);
    let breakdown = show_score_breakdown(&mode);

    let round_index = RwSignal::new(0usize);
    let score       = RwSignal::new(0u32);
    let selected    = RwSignal::new(Option::<Language>::None);
    let feedback    = RwSignal::new(Option::<GuessResponse>::None);
    let submitting  = RwSignal::new(false);
    let query       = RwSignal::new(String::new());

    let current_round = Memo::new(move |_| {
        rounds.with_value(|r| r[round_index.get()].clone())
    });

    let on_submit = Callback::new(move |_: leptos::web_sys::MouseEvent| {
        let Some(lang) = selected.get() else { return };
        let round_id = current_round.get().round_id;
        submitting.set(true);
        spawn_local(async move {
            match submit_guess(game_id, round_id, lang).await {
                Ok(result) => {
                    score.update(|s| *s += result.score.total);
                    feedback.set(Some(result));
                    submitting.set(false);
                }
                Err(_) => submitting.set(false),
            }
        });
    });

    let on_next = Callback::new(move |_: leptos::web_sys::MouseEvent| {
        let next = round_index.get() + 1;
        if next >= total {
            on_finish.run(score.get());
        } else {
            round_index.set(next);
            selected.set(None);
            feedback.set(None);
            query.set(String::new());
        }
    });

    view! {
        <div class="round">
            <div class="round-header">
                <p class="round-counter">
                    "Round " {move || round_index.get() + 1} " of " {total}
                </p>
                <p class="round-score">"Score " {move || score.get()}</p>
            </div>
            <div class="text-block">
                {move || current_round.get().text}
            </div>

            <Show when=move || feedback.get().is_none()>
                {move || {
                    let round = current_round.get();
                    view! {
                        <GuessInput
                            options=round.options.clone()
                            pool=pool
                            query=query
                            selected=selected
                            submitting=submitting
                            on_submit=on_submit
                        />
                    }
                }}
            </Show>

            <Show when=move || feedback.get().is_some()>
                {move || feedback.get().map(|f| view! {
                    <FeedbackPanel
                        feedback=f
                        breakdown=breakdown
                        round_index=round_index
                        total=total
                        on_next=on_next
                    />
                })}
            </Show>
        </div>
    }
}
