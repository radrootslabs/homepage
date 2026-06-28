use leptos::prelude::*;

use crate::i18n::{self, MessageKey};

#[component]
pub fn PageNetworkGraph() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let locale = i18n.locale_signal();
    let src = move || format!("/assets/i18n/{}/radroots_network_graph.svg", locale.get());
    let alt_i18n = i18n.clone();
    let alt = move || i18n::text(&alt_i18n, MessageKey::HomepageHomeHowItWorksDiagramAlt);

    view! {
        <img
            class="page-diagram"
            src=src
            alt=alt
        />
    }
}
