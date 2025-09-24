#![cfg(feature = "ssr")]

use crate::presentation::rest::{
    attribute_controller,
    post_collection_info_controller,
    post_controller,
    post_info_controller,
    post_type_info_controller,
    taxonomy_info_controller,
    term_info_controller,
    user_controller,
};
use actix_web::web::{scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    // Controllers that expect to be nested under /api
    cfg.service(scope("/api").configure(user_controller::routes));

    // Controllers that already register routes under /api/...
    cfg
        .configure(post_controller::routes)
        .configure(post_info_controller::routes)
        .configure(post_type_info_controller::routes)
        .configure(taxonomy_info_controller::routes_post_taxonomies)
        .configure(term_info_controller::routes_terms)
        .configure(attribute_controller::routes_attributes)
        .configure(attribute_controller::routes_attribute_values)
        .configure(post_collection_info_controller::routes);
}
