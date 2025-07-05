#![cfg(feature = "ssr")]
pub fn init_env() {
    dotenvy::dotenv().ok();
    env_logger::init();
}