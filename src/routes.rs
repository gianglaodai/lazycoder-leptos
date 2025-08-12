#![cfg(feature = "ssr")]

use crate::presentation::rest::{post_controller, user_controller};
use actix_web::web::scope;

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        scope("/api")
            .configure(user_controller::routes)
            .configure(post_controller::routes),
    );
}
