use leptos::prelude::*;

use crate::i18n::{self, MessageKey};

#[component]
pub fn NotFound() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();

    view! {
        <main class="page-main page-main-standalone">
            <section class="page-section">
                <p class="page-text">{i18n::text(&i18n, MessageKey::HomepageNotFoundBody)}</p>
            </section>
        </main>
    }
}
