use leptos::prelude::*;
use leptos_lucide_rs::{Check, ChevronDown};
use leptos_router::{NavigateOptions, hooks::use_navigate};

use crate::components::ui::{
    MenuContent, MenuItem, MenuItemIndicator, MenuItemKind, MenuRoot, MenuTrigger,
};
use crate::i18n;
use crate::routes::{LocalisedRouteContext, localised_path};

#[component]
pub fn PageLocaleMenu() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let locale = i18n.locale_signal();
    let current_locale = move || locale.get();
    let route = use_context::<LocalisedRouteContext>()
        .expect("locale menu should render inside a localised route");
    let navigate = use_navigate();
    let checked_index = Signal::derive(move || {
        let current = locale.get();
        i18n::SUPPORTED_LOCALES
            .iter()
            .position(|option_locale| *option_locale == current)
    });

    view! {
        <MenuRoot class="page-locale-menu" checked_index=checked_index>
            <MenuTrigger class="page-locale-button">
                <span>{current_locale}</span>
                <span class="page-locale-chevron">
                    <ChevronDown />
                </span>
            </MenuTrigger>
            <MenuContent class="page-locale-popover">
                {i18n::SUPPORTED_LOCALES
                    .iter()
                    .enumerate()
                    .map(|(index, option_locale)| {
                        let option_locale = *option_locale;
                        let target_path = localised_path(route.key, option_locale);
                        let option_navigate = navigate.clone();

                        view! {
                            <MenuItem
                                index=index
                                kind=MenuItemKind::Radio
                                label=option_locale.to_owned()
                                class="page-locale-option"
                                on_select=Callback::new(move |_| {
                                    option_navigate(
                                        target_path,
                                        NavigateOptions {
                                            resolve: false,
                                            ..NavigateOptions::default()
                                        },
                                    );
                                })
                            >
                                <span>{option_locale}</span>
                                <MenuItemIndicator index=index class="page-locale-check-slot">
                                    <span class="page-locale-check">
                                        <Check />
                                    </span>
                                </MenuItemIndicator>
                            </MenuItem>
                        }
                    })
                    .collect_view()}
            </MenuContent>
        </MenuRoot>
    }
}
