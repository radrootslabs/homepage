use leptos::prelude::*;
use mf2_i18n::leptos::RichTextRenderNode;

use crate::i18n::{self, HomepageHomeHowItWorksNoticeRichArgs, MessageKey};
use crate::layout::{Footer, Nav};

const RADROOTS_GIT_URL: &str = env!("RADROOTS_GIT_URL");

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
                    <a class="page-link" href=RADROOTS_GIT_URL>
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
            <Nav />
            <main class="page-main">
                <section class="page-section">
                    <p class="page-text">{i18n::text(&i18n, MessageKey::HomepageHomeIntroApplication)}</p>
                    <p class="page-text">{i18n::text(&i18n, MessageKey::HomepageHomeIntroInfrastructure)}</p>
                    <p class="page-text">{i18n::text(&i18n, MessageKey::HomepageHomeIntroFoundation)}</p>
                </section>
                <section class="page-section">
                    <p class="page-text">{i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksHeading)}</p>
                    <img
                        class="page-diagram"
                        src="/assets/radroots_network_graph.svg"
                        alt=i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksDiagramAlt)
                    />
                    <p class="page-text">
                        <span>{i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksStack)}</span>
                    </p>
                    <p class="page-text page-marker-row">
                        <span class="page-marker">{i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksNoticeMarker)}</span>
                        <span class="page-marker-body">
                            {notice}
                        </span>
                    </p>
                    <p class="page-text">
                        <span>{i18n::text(&i18n, MessageKey::HomepageHomeGettingStartedBody)}</span>
                    </p>
                    <p class="page-text page-bullet-row">
                        <img
                            class="page-bullet-mark"
                            src="/icons/lucide_asterisk.svg"
                            alt=""
                        />
                        <span class="page-bullet-body">
                            <a class="page-link page-inline-link page-plain-link" href="#">
                                <span>{i18n::text(&i18n, MessageKey::HomepageHomeDownloadIos)}" "</span>
                                <img
                                    class="page-inline-icon"
                                    src="/icons/lucide_arrow_up_right.svg"
                                    alt=""
                                />
                            </a>
                        </span>
                    </p>
                    <p class="page-text page-bullet-row">
                        <img
                            class="page-bullet-mark"
                            src="/icons/lucide_asterisk.svg"
                            alt=""
                        />
                        <span class="page-bullet-body">
                            <a class="page-link page-inline-link page-plain-link" href="#">
                                <span>{i18n::text(&i18n, MessageKey::HomepageHomeDownloadAndroid)}" "</span>
                                <img
                                    class="page-inline-icon"
                                    src="/icons/lucide_arrow_up_right.svg"
                                    alt=""
                                />
                            </a>
                        </span>
                    </p>
                    <p class="page-text">
                        <span>{i18n::text(&i18n, MessageKey::HomepageHomeDesktopBody)}</span>
                    </p>
                    <p class="page-text page-bullet-row">
                        <img
                            class="page-bullet-mark"
                            src="/icons/lucide_asterisk.svg"
                            alt=""
                        />
                        <span class="page-bullet-body">
                            <a class="page-link page-inline-link page-plain-link" href="#">
                                <span>{i18n::text(&i18n, MessageKey::HomepageHomeDownloadDesktop)}" "</span>
                                <img
                                    class="page-inline-icon"
                                    src="/icons/lucide_arrow_up_right.svg"
                                    alt=""
                                />
                            </a>
                        </span>
                    </p>
                    <p class="page-text">
                        <span>{i18n::text(&i18n, MessageKey::HomepageHomeCliBody)}</span>
                    </p>
                    <p class="page-text page-bullet-row">
                        <img
                            class="page-bullet-mark"
                            src="/icons/lucide_asterisk.svg"
                            alt=""
                        />
                        <span class="page-bullet-body">
                            <a class="page-link page-inline-link page-plain-link" href="#">
                                <span>{i18n::text(&i18n, MessageKey::HomepageHomeDownloadCli)}" "</span>
                                <img
                                    class="page-inline-icon"
                                    src="/icons/lucide_arrow_up_right.svg"
                                    alt=""
                                />
                            </a>
                        </span>
                    </p>
                </section>
            </main>
        </div>
        <Footer />
    }
}
