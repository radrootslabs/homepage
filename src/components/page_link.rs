use leptos::prelude::*;
use leptos_router::components::A;

fn is_page_route(href: &str) -> bool {
    href.starts_with('/') && !href.starts_with("//")
}

#[component]
pub(super) fn PageLink(
    href: &'static str,
    #[prop(default = "page-link")] class: &'static str,
    children: Children,
) -> AnyView {
    if is_page_route(href) {
        view! {
            <A attr:class=class href=href>
                {children()}
            </A>
        }
        .into_any()
    } else {
        view! {
            <a class=class href=href>
                {children()}
            </a>
        }
        .into_any()
    }
}
