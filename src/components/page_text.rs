use leptos::prelude::*;

use crate::i18n::{self, MessageKey};

#[component]
pub fn PageText(label: MessageKey) -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();

    view! {
        <p class="page-text">
            <span>{i18n::text(&i18n, label)}</span>
        </p>
    }
}
