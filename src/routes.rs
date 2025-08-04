#![cfg(feature = "ssr")]

use actix_web::web::scope;
use crate::presentation::rest::{post_controller, user_controller};

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/api")
            .configure(user_controller::routes)
            .configure(post_controller::routes)
    );
}
