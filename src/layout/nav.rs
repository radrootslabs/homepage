use leptos::prelude::*;
use leptos_router::components::A;

use crate::i18n::{self, MessageKey};

#[component]
pub fn PageNav() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();

    view! {
        <nav class="page-nav">
            <A attr:class="page-logo" href="/">
                <img
                    class="page-logo-image"
                    src="/assets/radroots_logotype_white.svg"
                    alt=i18n::text(&i18n, MessageKey::HomepagePageLogoAlt)
                />
            </A>
        </nav>
    }
}
