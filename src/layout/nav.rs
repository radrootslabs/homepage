use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="page-nav">
            <a class="page-logo" href="/">
                <img
                    class="page-logo-image"
                    src="/assets/radroots_logotype_white.svg"
                    alt="Radroots"
                />
            </a>
        </nav>
    }
}
