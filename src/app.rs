use leptos::prelude::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::routes::{
    Contact, Home, LocalisedFallback, LocalisedRoutePage, LocalisedRoutePath, NotFound, RouteKey,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! {
                <LocalisedFallback locale="en">
                    <NotFound />
                </LocalisedFallback>
            }>
                <Route
                    path=StaticSegment(LocalisedRoutePath::HomeEn)
                    view=|| view! {
                        <LocalisedRoutePage key=RouteKey::Home locale="en">
                            <Home />
                        </LocalisedRoutePage>
                    }
                />
                <Route
                    path=StaticSegment(LocalisedRoutePath::HomeEs)
                    view=|| view! {
                        <LocalisedRoutePage key=RouteKey::Home locale="es">
                            <Home />
                        </LocalisedRoutePage>
                    }
                />
                <Route
                    path=StaticSegment(LocalisedRoutePath::ContactEn)
                    view=|| view! {
                        <LocalisedRoutePage key=RouteKey::Contact locale="en">
                            <Contact />
                        </LocalisedRoutePage>
                    }
                />
                <Route
                    path=StaticSegment(LocalisedRoutePath::ContactEs)
                    view=|| view! {
                        <LocalisedRoutePage key=RouteKey::Contact locale="es">
                            <Contact />
                        </LocalisedRoutePage>
                    }
                />
            </Routes>
        </Router>
    }
}
