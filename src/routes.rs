#![cfg(feature = "ssr")]

use actix_web::web::scope;
use crate::presentation::rest::{post_api, user_api};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/api")
            .configure(user_api::routes)
            .configure(post_api::routes)
    );
}
