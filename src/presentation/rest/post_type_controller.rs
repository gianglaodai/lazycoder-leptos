#![cfg(feature = "ssr")]

use crate::business::post_type_service::{PostType, PostTypeCreate, PostTypeInfo};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_be, define_to_with_common_fields_be};
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};

// Table
define_to_with_common_fields_be!(PostType {
    pub code: String,
    pub name: String,
});

// View
define_readonly_to_with_common_fields_be!(PostTypeInfo {
    pub code: String,
    pub name: String,
});

impl From<PostTypeTO> for PostType {
    fn from(to: PostTypeTO) -> Self {
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
impl From<PostType> for PostTypeTO {
    fn from(e: PostType) -> Self {
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
impl From<PostTypeInfo> for PostTypeInfoTO {
    fn from(e: PostTypeInfo) -> Self {
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
            .post_type_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        PostTypeTO::from,
    )
}

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.post_type_service.count(query.to_filters()).await)
}

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_type_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostTypeTO::from),
    )
}

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_type_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostTypeTO::from),
    )
}

#[post("")]
pub async fn create(state: Data<AppState>, data: Json<PostTypeCreateTO>) -> impl Responder {
    respond_result(
        state
            .post_type_service
            .create(&PostTypeCreate::from(data.into_inner()))
            .await
            .map(PostTypeTO::from),
    )
}

#[put("/{id}")]
pub async fn update(state: Data<AppState>, data: Json<PostTypeTO>) -> impl Responder {
    respond_result(
        state
            .post_type_service
            .update(&PostType::from(data.into_inner()))
            .await
            .map(PostTypeTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(state.post_type_service.delete_by_id(id.into_inner()).await)
}

#[delete("/uid/{uid}")]
pub async fn delete_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(state.post_type_service.delete_by_uid(uid.into_inner()).await)
}

// Info endpoints
#[get("/info")]
pub async fn get_many_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .post_type_info_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        PostTypeInfoTO::from,
    )
}

#[get("/info/count")]
pub async fn count_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.post_type_info_service.count(query.to_filters()).await)
}

#[get("/{id}/info")]
pub async fn get_info_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_type_info_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostTypeInfoTO::from),
    )
}

#[get("/uid/{uid}/info")]
pub async fn get_info_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_type_info_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostTypeInfoTO::from),
    )
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/post_types")
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


impl From<PostTypeCreateTO> for PostTypeCreate {
    fn from(to: PostTypeCreateTO) -> Self {
        Self {
            code: to.code,
            name: to.name,
        }
    }
}
