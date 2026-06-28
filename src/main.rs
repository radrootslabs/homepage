mod app;
mod components;
mod config;
mod i18n;
mod nostr;
mod routes;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}
