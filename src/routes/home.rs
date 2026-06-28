use leptos::prelude::*;

use crate::layout::{Footer, Nav};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="page-shell">
            <Nav />
            <main class="page-main">
                <section class="page-section">
                    <p class="page-text">
                        "Radroots is an application for collaborative farming that operates across open-protocol networks."
                    </p>
                    <p class="page-text">
                        "Local food communities often rely on closed or centralized infrastructure that makes it difficult to build truly interoperable and community-owned networks."
                    </p>
                    <p class="page-text">
                        "Farmers and local food communities use Radroots to coordinate regional food systems, knowing that their activity is built on an open foundation."
                    </p>
                </section>
                <section class="page-section">
                    <p class="page-text">"- How it works -"</p>
                    <img
                        class="page-diagram"
                        src="/assets/radroots_network_graph.svg"
                        alt="Farmers, food hubs, local food routes, pickup points, restaurants, shops, families, and neighbors connected through Radroots."
                    />
                    <p class="page-text">
                        <span>"Radroots provides a complete application and networking stack that small-scale regional food networks can use for day-to-day organization and communication."</span>
                    </p>
                    <p class="page-text page-marker-row">
                        <span class="page-marker">"(!)"</span>
                        <span class="page-marker-body">
                            <span>"All of the source code for Radroots is "</span>
                            <a class="page-link" href="https://radroots.dev/git/"><span>"open source"</span></a>
                            <span>" and we welcome contributions of all kinds. Whether you are a developer, farmer, or have ideas you would like to share, we encourage you to "</span>
                            <a class="page-link" href="/contact"><span>"contact us"</span></a>
                            <span>" at any time."</span>
                        </span>
                    </p>
                    <p class="page-text">
                        <span>"Getting started requires a computer, cell phone, or server. Radroots has runtimes available for a diverse set of architectures (IoT coming soon), but for most people the best way to get started will be to download the iOS or Android mobile apps."</span>
                    </p>
                    <p class="page-text page-bullet-row">
                        <img
                            class="page-bullet-mark"
                            src="/icons/lucide_asterisk.svg"
                            alt=""
                        />
                        <span class="page-bullet-body">
                            <a class="page-link page-inline-link page-plain-link" href="#">
                                <span>"Radroots for iOS "</span>
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
                                <span>"Radroots for Android "</span>
                                <img
                                    class="page-inline-icon"
                                    src="/icons/lucide_arrow_up_right.svg"
                                    alt=""
                                />
                            </a>
                        </span>
                    </p>
                    <p class="page-text">
                        <span>"For anyone wanting to run Radroots on their desktop computer, you can download the Radroots application for Windows, macOS, and Linux operating systems."</span>
                    </p>
                    <p class="page-text page-bullet-row">
                        <img
                            class="page-bullet-mark"
                            src="/icons/lucide_asterisk.svg"
                            alt=""
                        />
                        <span class="page-bullet-body">
                            <a class="page-link page-inline-link page-plain-link" href="#">
                                <span>"Radroots for Desktop "</span>
                                <img
                                    class="page-inline-icon"
                                    src="/icons/lucide_arrow_up_right.svg"
                                    alt=""
                                />
                            </a>
                        </span>
                    </p>
                    <p class="page-text">
                        <span>"For users interested in scripting with Radroots, or using agentic workflows, Radroots has a command-line interface that can be used individually or concurrently with other Radroots applications."</span>
                    </p>
                    <p class="page-text page-bullet-row">
                        <img
                            class="page-bullet-mark"
                            src="/icons/lucide_asterisk.svg"
                            alt=""
                        />
                        <span class="page-bullet-body">
                            <a class="page-link page-inline-link page-plain-link" href="#">
                                <span>"Radroots CLI "</span>
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
