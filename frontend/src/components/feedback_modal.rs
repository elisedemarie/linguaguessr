use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::{submit_feedback, FeedbackPayload};
use crate::feedback_strings::{FEEDBACK_MODAL_TITLE, FEEDBACK_SUCCESS_MSG};
use crate::RoundContext;

#[component]
pub fn FeedbackModal(
    open:    RwSignal<bool>,
    context: RwSignal<Option<RoundContext>>,
) -> impl IntoView {
    let message    = RwSignal::new(String::new());
    let email      = RwSignal::new(String::new());
    let submitting = RwSignal::new(false);
    let error      = RwSignal::new(Option::<String>::None);
    let success    = RwSignal::new(false);

    let close = move || {
        open.set(false);
        message.set(String::new());
        email.set(String::new());
        error.set(None);
        success.set(false);
        submitting.set(false);
    };

    let on_submit = move |_: leptos::web_sys::MouseEvent| {
        let msg = message.get();
        if msg.trim().is_empty() {
            error.set(Some("Message is required.".into()));
            return;
        }
        error.set(None);
        submitting.set(true);

        let ctx = context.get();
        let email_val = email.get();

        spawn_local(async move {
            let payload = FeedbackPayload {
                message:     msg,
                email:       if email_val.trim().is_empty() { None } else { Some(email_val) },
                game_id:     ctx.as_ref().map(|c| c.game_id),
                round_id:    ctx.as_ref().map(|c| c.round_id),
                language:    ctx.as_ref().and_then(|c| c.language.clone()),
                article_url: None,
            };
            match submit_feedback(&payload).await {
                Ok(())  => { success.set(true); submitting.set(false); }
                Err(e)  => { error.set(Some(e)); submitting.set(false); }
            }
        });
    };

    view! {
        <Show when=move || open.get()>
            <div class="modal-overlay" on:click=move |_| close()>
                <div class="modal" on:click=|ev| ev.stop_propagation()>
                    <div class="modal-header">
                        <h2 class="modal-title">{FEEDBACK_MODAL_TITLE}</h2>
                        <button class="modal-close" on:click=move |_| close()>"×"</button>
                    </div>

                    <Show
                        when=move || success.get()
                        fallback=move || view! {
                            <div class="modal-body">
                                <div class="modal-field">
                                    <label class="modal-label">"Message"</label>
                                    <textarea
                                        class="modal-textarea"
                                        placeholder="Describe the issue…"
                                        prop:value=move || message.get()
                                        on:input=move |ev| message.set(event_target_value(&ev))
                                    />
                                </div>
                                <div class="modal-field">
                                    <label class="modal-label">"Email (optional)"</label>
                                    <input
                                        type="email"
                                        class="modal-input"
                                        placeholder="you@example.com"
                                        prop:value=move || email.get()
                                        on:input=move |ev| email.set(event_target_value(&ev))
                                    />
                                </div>
                                <Show when=move || error.get().is_some()>
                                    <p class="modal-error">{move || error.get().unwrap_or_default()}</p>
                                </Show>
                                <button
                                    class="modal-submit"
                                    on:click=on_submit
                                    prop:disabled=move || submitting.get()
                                >
                                    {move || if submitting.get() { "Sending…" } else { "Send" }}
                                </button>
                            </div>
                        }
                    >
                        <div class="modal-body modal-success">
                            <p>{FEEDBACK_SUCCESS_MSG}</p>
                            <button class="modal-submit" on:click=move |_| close()>"Close"</button>
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
    }
}
