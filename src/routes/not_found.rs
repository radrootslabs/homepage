use leptos::prelude::*;

use crate::components::PageText;
use crate::i18n::MessageKey;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <main class="page-main page-main-standalone">
            <section class="page-section">
                <PageText label=MessageKey::HomepageNotFoundBody />
            </section>
        </main>
    }
}
