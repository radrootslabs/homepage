use leptos::prelude::*;

use crate::i18n::{self, MessageKey};
use crate::layout::{Footer, Nav};

#[component]
pub fn Contact() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();

    view! {
        <div class="page-shell">
            <Nav />
            <main class="page-main">
                <section class="page-section">
                    <p class="page-text">{i18n::text(&i18n, MessageKey::HomepageContactBody)}</p>
                </section>
            </main>
        </div>
        <Footer />
    }
}
