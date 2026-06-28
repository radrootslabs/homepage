use leptos::prelude::*;

use crate::components::{PageLayout, PageSection, PageText};
use crate::i18n::MessageKey;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <PageLayout>
            <PageSection>
                <PageText label=MessageKey::HomepageContactBody />
            </PageSection>
        </PageLayout>
    }
}
