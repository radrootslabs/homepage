use leptos::prelude::*;

#[component]
pub fn PageSection(children: Children) -> impl IntoView {
    view! {
        <section class="page-section">
            {children()}
        </section>
    }
}
