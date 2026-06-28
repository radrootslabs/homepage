use leptos::prelude::*;

use crate::components::PageText;
use crate::i18n::MessageKey;
use crate::layout::{PageFooter, PageNav};

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <div class="page-shell">
            <PageNav />
            <main class="page-main">
                <section class="page-section">
                    <PageText label=MessageKey::HomepageContactBody />
                </section>
            </main>
        </div>
        <PageFooter />
    }
}
