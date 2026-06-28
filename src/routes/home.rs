use leptos::prelude::*;

use crate::i18n::{self, MessageKey};
use crate::layout::{Footer, Nav};

#[component]
pub fn Home() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();

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
                            <span>{i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksNoticePrefix)}</span>
                            " "
                            <a class="page-link" href="https://radroots.dev/git/"><span>{i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksNoticeOpenSourceLabel)}</span></a>
                            " "
                            <span>{i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksNoticeMiddle)}</span>
                            " "
                            <a class="page-link" href="/contact"><span>{i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksNoticeContactLabel)}</span></a>
                            " "
                            <span>{i18n::text(&i18n, MessageKey::HomepageHomeHowItWorksNoticeSuffix)}</span>
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
