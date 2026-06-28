use leptos::prelude::*;

use super::page_link::PageLink;
use crate::i18n::{self, MessageKey};

#[component]
pub fn PageNav() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();

    view! {
        <nav class="page-nav">
            <PageLink href="/" class="page-logo">
                <img
                    class="page-logo-image"
                    src="/assets/radroots_logotype_white.svg"
                    alt=i18n::text(&i18n, MessageKey::HomepagePageLogoAlt)
                />
            </PageLink>
        </nav>
    }
}
