use leptos::prelude::*;

mod path;

pub use path::{LocalisedRoutePath, RouteKey, localised_path};

use crate::i18n;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalisedRouteContext {
    pub key: RouteKey,
    pub locale: &'static str,
}

#[component]
pub fn LocalisedRoutePage(
    key: RouteKey,
    locale: &'static str,
    children: Children,
) -> impl IntoView {
    let i18n = mf2_i18n::leptos::provide_i18n_with_locale(i18n::localizer(), locale)
        .expect("localised route locale should be supported");
    provide_context(LocalisedRouteContext { key, locale });
    let _document_effects = i18n::sync_leptos_document(&i18n);

    view! {
        {children()}
    }
}

#[component]
pub fn LocalisedFallback(locale: &'static str, children: Children) -> impl IntoView {
    let i18n = mf2_i18n::leptos::provide_i18n_with_locale(i18n::localizer(), locale)
        .expect("localised fallback locale should be supported");
    let _document_effects = i18n::sync_leptos_document(&i18n);

    view! {
        {children()}
    }
}
