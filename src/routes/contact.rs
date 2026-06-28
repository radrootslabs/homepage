use leptos::prelude::*;

use crate::layout::{Footer, Nav};

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <div class="page-shell">
            <Nav />
            <main class="page-main">
                <section class="page-section">
                    <p class="page-text">"contact"</p>
                </section>
            </main>
        </div>
        <Footer />
    }
}
