use leptos::prelude::*;

use super::page_link::PageLink;
use crate::i18n::{self, MessageKey};

#[component]
pub fn PageTextLink(href: &'static str, label: MessageKey) -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let text = i18n::text(&i18n, label);

    view! {
        <PageLink href=href>
            <span>{text}</span>
        </PageLink>
    }
}
