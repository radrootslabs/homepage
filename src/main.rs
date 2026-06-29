mod app;
mod components;
mod config;
mod i18n;
mod nostr;
mod routes;
mod support_contact;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}
