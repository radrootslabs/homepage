use leptos::prelude::*;

use crate::components::{PageFooter, PageNav};

#[component]
pub fn PageLayout(children: Children) -> impl IntoView {
    view! {
        <div class="page-shell">
            <PageNav />
            <main class="page-main">
                {children()}
            </main>
        </div>
        <PageFooter />
    }
}
