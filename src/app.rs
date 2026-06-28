use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::i18n;
use crate::routes::{Contact, Home, NotFound};

#[component]
pub fn App() -> impl IntoView {
    let i18n = mf2_i18n::leptos::provide_i18n(i18n::localizer());
    let _document_effects = i18n::sync_leptos_document(&i18n);

    view! {
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/") view=Home />
                <Route path=path!("/contact") view=Contact />
            </Routes>
        </Router>
    }
}
