use leptos::prelude::*;
use leptos_lucide_rs::LoaderCircle;

#[component]
pub fn PageLoader() -> impl IntoView {
    view! {
        <span class="page-loader">
            <LoaderCircle />
        </span>
    }
}
