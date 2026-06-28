use leptos::prelude::*;

use crate::i18n::{self, MessageKey};

#[component]
pub fn PageMarkerText(marker: MessageKey, children: Children) -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let marker_i18n = i18n.clone();
    let marker_text = move || i18n::text(&marker_i18n, marker);

    view! {
        <p class="page-text page-marker-row">
            <span class="page-marker">{marker_text}</span>
            <span class="page-marker-body">
                {children()}
            </span>
        </p>
    }
}
