#![cfg(feature = "ssr")]

use crate::presentation::rest::{
    attribute_controller, attribute_value_controller, post_collection_controller, post_controller,
    post_taxonomy_controller, post_type_controller, term_controller, user_controller,
};
use actix_web::web::{scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    // Controllers that expect to be nested under /api
    cfg.service(scope("/api").configure(user_controller::routes));

    // Controllers that already register routes under /api/...
    cfg.configure(post_controller::routes)
        .configure(post_type_controller::routes)
        .configure(post_taxonomy_controller::routes)
        .configure(term_controller::routes)
        .configure(attribute_controller::routes_attributes)
        .configure(attribute_value_controller::routes)
        .configure(post_collection_controller::routes);
}
