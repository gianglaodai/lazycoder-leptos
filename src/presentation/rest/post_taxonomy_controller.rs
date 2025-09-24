#![cfg(feature = "ssr")]

use crate::business::post_taxonomy_service::{
    PostTaxonomy, PostTaxonomyCreate, PostTaxonomyInfo,
};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_be, define_to_with_common_fields_be};
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};

// Table
define_to_with_common_fields_be!(PostTaxonomy { pub code: String, pub name: String, });
// View
define_readonly_to_with_common_fields_be!(PostTaxonomyInfo { pub code: String, pub name: String, });

impl From<PostTaxonomyTO> for PostTaxonomy {
    fn from(to: PostTaxonomyTO) -> Self {
        Self {
            id: to.id,
            uid: to.uid,
            version: to.version,
            created_at: to.created_at,
            updated_at: to.updated_at,
            code: to.code,
            name: to.name,
        }
    }
}
impl From<PostTaxonomy> for PostTaxonomyTO {
    fn from(e: PostTaxonomy) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            code: e.code,
            name: e.name,
        }
    }
}
impl From<PostTaxonomyInfo> for PostTaxonomyInfoTO {
    fn from(e: PostTaxonomyInfo) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            code: e.code,
            name: e.name,
        }
    }
}

// Table endpoints
#[get("")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .post_taxonomy_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        PostTaxonomyTO::from,
    )
}

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.post_taxonomy_service.count(query.to_filters()).await)
}

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_taxonomy_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostTaxonomyTO::from),
    )
}

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_taxonomy_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostTaxonomyTO::from),
    )
}

#[post("")]
pub async fn create(state: Data<AppState>, data: Json<PostTaxonomyCreateTO>) -> impl Responder {
    respond_result(
        state
            .post_taxonomy_service
            .create(&PostTaxonomyCreate::from(data.into_inner()))
            .await
            .map(PostTaxonomyTO::from),
    )
}

#[put("/{id}")]
pub async fn update(state: Data<AppState>, data: Json<PostTaxonomyTO>) -> impl Responder {
    respond_result(
        state
            .post_taxonomy_service
            .update(&PostTaxonomy::from(data.into_inner()))
            .await
            .map(PostTaxonomyTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(state.post_taxonomy_service.delete_by_id(id.into_inner()).await)
}

#[delete("/uid/{uid}")]
pub async fn delete_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(state.post_taxonomy_service.delete_by_uid(uid.into_inner()).await)
}

// Info endpoints
#[get("/info")]
pub async fn get_many_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .post_taxonomy_info_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        PostTaxonomyInfoTO::from,
    )
}

#[get("/info/count")]
pub async fn count_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.post_taxonomy_info_service.count(query.to_filters()).await)
}

#[get("/{id}/info")]
pub async fn get_info_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_taxonomy_info_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostTaxonomyInfoTO::from),
    )
}

#[get("/uid/{uid}/info")]
pub async fn get_info_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_taxonomy_info_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostTaxonomyInfoTO::from),
    )
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/post_taxonomies")
            .service(get_many)
            .service(count)
            .service(get_by_id)
            .service(get_by_uid)
            .service(create)
            .service(update)
            .service(delete_by_id)
            .service(delete_by_uid)
            .service(get_many_info)
            .service(count_info)
            .service(get_info_by_id)
            .service(get_info_by_uid),
    );
}


impl From<PostTaxonomyCreateTO> for PostTaxonomyCreate {
    fn from(to: PostTaxonomyCreateTO) -> Self {
        Self { code: to.code, name: to.name }
    }
}
