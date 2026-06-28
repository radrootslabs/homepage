use leptos::prelude::*;

use crate::components::{PageSection, PageText};
use crate::i18n::MessageKey;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <main class="page-main page-main-standalone">
            <PageSection>
                <PageText label=MessageKey::HomepageNotFoundBody />
            </PageSection>
        </main>
    }
}
