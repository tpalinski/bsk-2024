pub mod app;

#[cfg(feature = "ssr")]
pub mod rsa;

#[cfg(feature = "ssr")]
pub mod keys;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
