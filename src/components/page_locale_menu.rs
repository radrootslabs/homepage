use leptos::prelude::*;
use leptos_router::{NavigateOptions, hooks::use_navigate};

use crate::i18n;
use crate::routes::{LocalisedRouteContext, localised_path};

const POPOVER_ID: &str = "page-locale-menu-popover";

#[component]
pub fn PageLocaleMenu() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let locale = i18n.locale_signal();
    let current_locale = move || locale.get();
    let route = use_context::<LocalisedRouteContext>()
        .expect("locale menu should render inside a localised route");
    let navigate = use_navigate();

    view! {
        <div class="page-locale-menu">
            <button
                class="page-locale-button"
                type="button"
                popovertarget=POPOVER_ID
            >
                <span>{current_locale}</span>
                <span class="page-locale-chevron"></span>
            </button>
            <div
                class="page-locale-popover"
                id=POPOVER_ID
                popover="auto"
            >
                <div class="page-locale-list">
                    {i18n::SUPPORTED_LOCALES
                        .iter()
                        .map(|option_locale| {
                            let option_locale = *option_locale;
                            let target_path = localised_path(route.key, option_locale);
                            let option_navigate = navigate.clone();
                            let option_signal = locale;
                            let check_signal = locale;

                            view! {
                                <button
                                    class="page-locale-option"
                                    class:page-locale-option-active=move || option_signal.get() == option_locale
                                    type="button"
                                    popovertarget=POPOVER_ID
                                    popovertargetaction="hide"
                                    on:click=move |_| {
                                        option_navigate(
                                            target_path,
                                            NavigateOptions {
                                                resolve: false,
                                                ..NavigateOptions::default()
                                            },
                                        );
                                    }
                                >
                                    <span>{option_locale}</span>
                                    <span class="page-locale-check-slot">
                                        {move || {
                                            (check_signal.get() == option_locale).then_some(view! {
                                                <span class="page-locale-check"></span>
                                            })
                                        }}
                                    </span>
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </div>
    }
}
