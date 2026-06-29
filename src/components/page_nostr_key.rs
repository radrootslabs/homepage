use leptos::prelude::*;

use crate::features::nostr::browser;

#[component]
pub fn PageNostrKey() -> impl IntoView {
    let public_key = LocalResource::new(browser::load_public_key);

    view! {
        {move || {
            public_key
                .get()
                .flatten()
                .map(|public_key| {
                    view! {
                        <p class="page-nav-public-key">{public_key}</p>
                    }
                })
        }}
    }
}
