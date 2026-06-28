use leptos::prelude::*;

use crate::i18n::{self, MessageKey};

#[component]
pub fn PageMarkerText(marker: MessageKey, children: Children) -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();

    view! {
        <p class="page-text page-marker-row">
            <span class="page-marker">{i18n::text(&i18n, marker)}</span>
            <span class="page-marker-body">
                {children()}
            </span>
        </p>
    }
}
