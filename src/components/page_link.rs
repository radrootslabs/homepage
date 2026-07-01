use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::ui::Anchor;
use crate::routes::{LocalisedRouteContext, RouteKey, localised_path};

#[derive(Clone, Copy)]
pub enum PageHref {
    Route(RouteKey),
    External(&'static str),
}

#[component]
pub(super) fn PageLink(
    href: PageHref,
    #[prop(default = "page-link")] class: &'static str,
    children: Children,
) -> AnyView {
    match href {
        PageHref::Route(key) => {
            let route = use_context::<LocalisedRouteContext>()
                .expect("route links should render inside a localised route");
            let href = localised_path(key, route.locale).to_owned();

            view! {
                <A attr:class=class href=href>
                    {children()}
                </A>
            }
            .into_any()
        }
        PageHref::External(href) => view! {
            <Anchor class=class href=href>
                {children()}
            </Anchor>
        }
        .into_any(),
    }
}
