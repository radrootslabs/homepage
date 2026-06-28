use leptos::prelude::*;

use crate::i18n::{self, MessageKey};

#[component]
pub fn PageFooter() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let brand_i18n = i18n.clone();
    let brand = move || i18n::text(&brand_i18n, MessageKey::HomepagePageFooterBrand);
    let tagline_i18n = i18n.clone();
    let tagline = move || i18n::text(&tagline_i18n, MessageKey::HomepagePageFooterTagline);

    view! {
        <footer class="page-footer">
            <div class="page-footer-bar">
                <p class="page-text page-footer-text">{brand}</p>
                <p class="page-text page-footer-text">{tagline}</p>
            </div>
        </footer>
    }
}
