//! Cipher Praxis — A Dilate Cryptography Knowledge Base. Client-side Leptos application.
mod app;
mod components;
mod labs;
mod pages;
mod state;
mod util;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
