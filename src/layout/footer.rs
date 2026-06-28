use leptos::prelude::*;

use crate::i18n::{self, MessageKey};

#[component]
pub fn PageFooter() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();

    view! {
        <footer class="page-footer">
            <div class="page-footer-bar">
                <p class="page-text page-footer-text">{i18n::text(&i18n, MessageKey::HomepagePageFooterBrand)}</p>
                <p class="page-text page-footer-text">{i18n::text(&i18n, MessageKey::HomepagePageFooterTagline)}</p>
            </div>
        </footer>
    }
}
