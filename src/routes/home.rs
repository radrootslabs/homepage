use leptos::prelude::*;
use mf2_i18n::leptos::RichTextRenderNode;

use crate::components::{
    PageBulletLink, PageLayout, PageMarkerText, PageSection, PageText, PageTextLink,
};
use crate::config;
use crate::i18n::{self, HomepageHomeHowItWorksNoticeRichArgs, MessageKey};

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
                    <PageTextLink href=config::RADROOTS_GIT_URL label=MessageKey::HomepageHomeHowItWorksNoticeOpenSourceLabel />
                }
                .into_any(),
                "contact" => view! {
                    <PageTextLink href="/contact" label=MessageKey::HomepageHomeHowItWorksNoticeContactLabel />
                }
                .into_any(),
                _ => unreachable!("unknown homepage notice rich text slot"),
            },
        })
        .collect::<Vec<_>>();

    view! {
        <PageLayout>
            <PageSection>
                <PageText label=MessageKey::HomepageHomeIntroApplication />
                <PageText label=MessageKey::HomepageHomeIntroInfrastructure />
                <PageText label=MessageKey::HomepageHomeIntroFoundation />
            </PageSection>
            <PageSection>
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
            </PageSection>
        </PageLayout>
    }
}
