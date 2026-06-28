use leptos::prelude::*;

use super::page_link::PageLink;
use crate::i18n::{self, MessageKey};

#[component]
pub fn PageNav() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let logo_i18n = i18n.clone();
    let logo_alt = move || i18n::text(&logo_i18n, MessageKey::HomepagePageLogoAlt);

    view! {
        <nav class="page-nav">
            <PageLink href="/" class="page-logo">
                <img
                    class="page-logo-image"
                    src="/assets/radroots_logotype_white.svg"
                    alt=logo_alt
                />
            </PageLink>
        </nav>
    }
}
