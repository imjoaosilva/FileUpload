use leptos::*;

use crate::app::App;

mod app;
mod services;

fn main() {
    mount_to_body(|| view! { <App/> });
}
