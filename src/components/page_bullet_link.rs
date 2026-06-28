use leptos::prelude::*;

use crate::i18n::{self, MessageKey};

#[component]
pub fn PageBulletLink(href: &'static str, label: MessageKey) -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();

    view! {
        <p class="page-text page-bullet-row">
            <img
                class="page-bullet-mark"
                src="/icons/lucide_asterisk.svg"
                alt=""
            />
            <span class="page-bullet-body">
                <a class="page-link page-inline-link page-plain-link" href=href>
                    <span>{i18n::text(&i18n, label)}" "</span>
                    <img
                        class="page-inline-icon"
                        src="/icons/lucide_arrow_up_right.svg"
                        alt=""
                    />
                </a>
            </span>
        </p>
    }
}
