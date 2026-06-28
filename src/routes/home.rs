use leptos::prelude::*;
use mf2_i18n::leptos::RichTextRenderNode;

use crate::components::{PageBulletLink, PageMarkerText, PageText};
use crate::config;
use crate::i18n::{self, HomepageHomeHowItWorksNoticeRichArgs, MessageKey};
use crate::layout::{PageFooter, PageNav};

#[component]
pub fn Home() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let notice_args = HomepageHomeHowItWorksNoticeRichArgs::new((), ());
    let notice = i18n::rich_homepage_home_how_it_works_notice(&i18n, &notice_args)
        .into_iter()
        .map(|node| match node {
            RichTextRenderNode::Text(text) => view! { <span>{text}</span> }.into_any(),
            RichTextRenderNode::Slot { name, .. } => match name.as_str() {
                "open_source" => view! {
                    <a class="page-link" href=config::RADROOTS_GIT_URL>
                        <span>{i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksNoticeOpenSourceLabel)}</span>
                    </a>
                }
                .into_any(),
                "contact" => view! {
                    <a class="page-link" href="/contact">
                        <span>{i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksNoticeContactLabel)}</span>
                    </a>
                }
                .into_any(),
                _ => unreachable!("unknown homepage notice rich text slot"),
            },
        })
        .collect::<Vec<_>>();

    view! {
        <div class="page-shell">
            <PageNav />
            <main class="page-main">
                <section class="page-section">
                    <PageText label=MessageKey::HomepageHomeIntroApplication />
                    <PageText label=MessageKey::HomepageHomeIntroInfrastructure />
                    <PageText label=MessageKey::HomepageHomeIntroFoundation />
                </section>
                <section class="page-section">
                    <PageText label=MessageKey::HomepageHomeHowItWorksHeading />
                    <img
                        class="page-diagram"
                        src="/assets/radroots_network_graph.svg"
                        alt=i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksDiagramAlt)
                    />
                    <PageText label=MessageKey::HomepageHomeHowItWorksStack />
                    <PageMarkerText marker=MessageKey::HomepageHomeHowItWorksNoticeMarker>
                        {notice}
                    </PageMarkerText>
                    <PageText label=MessageKey::HomepageHomeGettingStartedBody />
                    <PageBulletLink href=config::RADROOTS_IOS_URL label=MessageKey::HomepageHomeDownloadIos />
                    <PageBulletLink href=config::RADROOTS_ANDROID_URL label=MessageKey::HomepageHomeDownloadAndroid />
                    <PageText label=MessageKey::HomepageHomeDesktopBody />
                    <PageBulletLink href=config::RADROOTS_DESKTOP_URL label=MessageKey::HomepageHomeDownloadDesktop />
                    <PageText label=MessageKey::HomepageHomeCliBody />
                    <PageBulletLink href=config::RADROOTS_CLI_URL label=MessageKey::HomepageHomeDownloadCli />
                </section>
            </main>
        </div>
        <PageFooter />
    }
}
