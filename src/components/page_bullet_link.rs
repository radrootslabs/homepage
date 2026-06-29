use leptos::prelude::*;
use leptos_lucide_rs::{ArrowUpRight, Asterisk};

use super::page_link::{PageHref, PageLink};
use crate::i18n::{self, MessageKey};

#[component]
pub fn PageBulletLink(href: &'static str, label: MessageKey) -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let text_i18n = i18n.clone();
    let text = move || i18n::text(&text_i18n, label);

    view! {
        <p class="page-text page-bullet-row">
            <span class="page-bullet-mark">
                <Asterisk />
            </span>
            <span class="page-bullet-body">
                <PageLink href=PageHref::External(href) class="page-link page-inline-link page-plain-link">
                    <span>{text}" "</span>
                    <span class="page-inline-icon">
                        <ArrowUpRight />
                    </span>
                </PageLink>
            </span>
        </p>
    }
}
