mod app;
mod layout;
mod routes;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}
