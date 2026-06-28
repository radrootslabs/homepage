use leptos::prelude::*;

use super::page_link::PageLink;
use crate::i18n::{self, MessageKey};

#[component]
pub fn PageBulletLink(href: &'static str, label: MessageKey) -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let text_i18n = i18n.clone();
    let text = move || i18n::text(&text_i18n, label);

    view! {
        <p class="page-text page-bullet-row">
            <img
                class="page-bullet-mark"
                src="/icons/lucide_asterisk.svg"
                alt=""
            />
            <span class="page-bullet-body">
                <PageLink href=href class="page-link page-inline-link page-plain-link">
                    <span>{text}" "</span>
                    <img
                        class="page-inline-icon"
                        src="/icons/lucide_arrow_up_right.svg"
                        alt=""
                    />
                </PageLink>
            </span>
        </p>
    }
}
