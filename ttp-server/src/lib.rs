pub mod app;
pub mod pages;
pub mod model;
#[cfg(feature = "ssr")]
pub mod crypto_server;
#[cfg(feature = "ssr")]
pub mod user_repository;
#[cfg(feature = "ssr")]
pub mod key_api;
#[cfg(feature = "ssr")]
pub mod user_api;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
