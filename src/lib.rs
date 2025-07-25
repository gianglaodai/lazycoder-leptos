#[cfg(feature = "ssr")]
pub mod app;
#[cfg(feature = "ssr")]
pub mod config;
pub mod pages;
#[cfg(feature = "ssr")]
pub mod routes;
#[cfg(feature = "ssr")]
pub mod state;
#[cfg(feature = "ssr")]
pub mod presentation;
#[cfg(feature = "ssr")]
pub mod infras;
pub mod business;
#[cfg(feature = "ssr")]
pub mod db;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(pages::app_component::App);
}
