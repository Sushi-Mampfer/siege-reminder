pub mod app;
pub mod datatypes;
pub mod db;
pub mod pages;
pub mod server;
pub mod notifications;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

pub use app::*;
pub use pages::*;
pub use server::*;
