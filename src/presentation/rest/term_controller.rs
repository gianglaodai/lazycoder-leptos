#![cfg(feature = "ssr")]

use crate::business::term_service::{Term, TermCreate, TermInfo};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_be, define_to_with_common_fields_be};
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};

// Table
define_to_with_common_fields_be!(Term {
    req {
        pub taxonomy_id: i32,
        pub slug: String,
        pub name: String,
    }
    opt {
        pub parent_id: Option<i32>,
        pub description: Option<String>,
    }
});

// View
define_readonly_to_with_common_fields_be!(TermInfo {
    pub taxonomy_id: i32,
    pub taxonomy_code: String,
    pub taxonomy_name: String,
    pub parent_id: Option<i32>,
    pub parent_slug: Option<String>,
    pub parent_name: Option<String>,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
});

impl From<TermTO> for Term {
    fn from(to: TermTO) -> Self {
        Self {
            id: to.id,
            uid: to.uid,
            version: to.version,
            created_at: to.created_at,
            updated_at: to.updated_at,
            taxonomy_id: to.taxonomy_id,
            slug: to.slug,
            name: to.name,
            parent_id: to.parent_id,
            description: to.description,
        }
    }
}
impl From<Term> for TermTO {
    fn from(e: Term) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            taxonomy_id: e.taxonomy_id,
            slug: e.slug,
            name: e.name,
            parent_id: e.parent_id,
            description: e.description,
        }
    }
}
impl From<TermInfo> for TermInfoTO {
    fn from(e: TermInfo) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            taxonomy_id: e.taxonomy_id,
            taxonomy_code: e.taxonomy_code,
            taxonomy_name: e.taxonomy_name,
            parent_id: e.parent_id,
            parent_slug: e.parent_slug,
            parent_name: e.parent_name,
            slug: e.slug,
            name: e.name,
            description: e.description,
        }
    }
}

// Table endpoints
#[get("")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .term_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        TermTO::from,
    )
}

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.term_service.count(query.to_filters()).await)
}

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .term_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(TermTO::from),
    )
}

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .term_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(TermTO::from),
    )
}

#[post("")]
pub async fn create(state: Data<AppState>, data: Json<TermCreateTO>) -> impl Responder {
    respond_result(
        state
            .term_service
            .create(&TermCreate::from(data.into_inner()))
            .await
            .map(TermTO::from),
    )
}

#[put("/{id}")]
pub async fn update(state: Data<AppState>, data: Json<TermTO>) -> impl Responder {
    respond_result(
        state
            .term_service
            .update(&Term::from(data.into_inner()))
            .await
            .map(TermTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(state.term_service.delete_by_id(id.into_inner()).await)
}

#[delete("/uid/{uid}")]
pub async fn delete_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(state.term_service.delete_by_uid(uid.into_inner()).await)
}

// Info endpoints
#[get("/info")]
pub async fn get_many_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .term_info_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        TermInfoTO::from,
    )
}

#[get("/info/count")]
pub async fn count_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.term_info_service.count(query.to_filters()).await)
}

#[get("/{id}/info")]
pub async fn get_info_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .term_info_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(TermInfoTO::from),
    )
}

#[get("/uid/{uid}/info")]
pub async fn get_info_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .term_info_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(TermInfoTO::from),
    )
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/terms")
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


impl From<TermCreateTO> for TermCreate {
    fn from(to: TermCreateTO) -> Self {
        Self {
            taxonomy_id: to.taxonomy_id,
            slug: to.slug,
            name: to.name,
        }
    }
}
