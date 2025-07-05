#![cfg(feature = "ssr")]
use crate::presentation::rest::user_rest_api;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    user_rest_api::routes(cfg);
}
