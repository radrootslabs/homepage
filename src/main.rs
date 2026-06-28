use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="page-shell">
            <nav class="page-nav">
                <a class="page-logo" href="/">
                    <img
                        class="page-logo-image"
                        src="/assets/radroots_logotype_white.svg"
                        alt="Radroots"
                    />
                </a>
            </nav>
            <main class="page-main">
                <section class="page-section">
                    <p class="page-text">
                        "Radroots is an application for collaborative farming that operates across open-protocol networks."
                    </p>
                    <p class="page-text">
                        "Local food communities often rely on closed or centralized infrastructure that makes it difficult to build truly interoperable and community-owned networks."
                    </p>
                    <p class="page-text">
                        "Farmers and local food communities use Radroots to coordinate regional food systems, knowing that their activity is built on an open foundation."
                    </p>
                </section>
            </main>
        </div>
        <footer class="page-footer">
            <div class="page-footer-bar">
                <p class="page-text page-footer-text">"Radroots"</p>
                <p class="page-text page-footer-text">"People, food, planet."</p>
            </div>
        </footer>
    }
}
