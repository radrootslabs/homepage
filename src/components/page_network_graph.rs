use leptos::prelude::*;

use crate::i18n::{self, MessageKey};

const NETWORK_GRAPH_FILENAME: &str = "radroots_network_graph.svg";

#[component]
pub fn PageNetworkGraph() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let locale = i18n.locale_signal();
    let src = move || network_graph_src(&locale.get());
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

fn network_graph_src(locale: &str) -> String {
    format!("/assets/i18n/{locale}/{NETWORK_GRAPH_FILENAME}")
}
