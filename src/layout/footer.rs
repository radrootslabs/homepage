use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="page-footer">
            <div class="page-footer-bar">
                <p class="page-text page-footer-text">"Radroots"</p>
                <p class="page-text page-footer-text">"People, food, planet."</p>
            </div>
        </footer>
    }
}
