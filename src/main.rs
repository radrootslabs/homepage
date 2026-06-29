mod app;
mod components;
mod config;
mod features;
mod i18n;
mod routes;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}
