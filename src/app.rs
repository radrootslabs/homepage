use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::routes::{Contact, Home, NotFound};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=Home />
                <Route path=path!("/contact") view=Contact />
            </Routes>
        </Router>
    }
}
