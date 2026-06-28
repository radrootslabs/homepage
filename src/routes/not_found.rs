use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <main class="page-main page-main-standalone">
            <section class="page-section">
                <p class="page-text">"not found"</p>
            </section>
        </main>
    }
}
