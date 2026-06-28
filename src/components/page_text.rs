use leptos::prelude::*;

use crate::i18n::{self, MessageKey};

#[component]
pub fn PageText(label: MessageKey) -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let text_i18n = i18n.clone();
    let text = move || i18n::text(&text_i18n, label);

    view! {
        <p class="page-text">
            <span>{text}</span>
        </p>
    }
}
