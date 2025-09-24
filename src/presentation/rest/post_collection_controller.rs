#![cfg(feature = "ssr")]

use crate::business::post_collection_service::{
    PostCollection, PostCollectionCreate, PostCollectionInfo,
};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use crate::{define_readonly_to_with_common_fields_be, define_to_with_common_fields_be};
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};

// Table TO (CRUD)
define_to_with_common_fields_be!(PostCollection {
    req {
        pub slug: String,
        pub title: String,
        pub visibility: String,
    }
    opt {
        pub description: Option<String>,
    }
});

// View TO (info)
define_readonly_to_with_common_fields_be!(PostCollectionInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub visibility: String,
});

impl From<PostCollectionTO> for PostCollection {
    fn from(to: PostCollectionTO) -> Self {
        Self {
            id: to.id,
            uid: to.uid,
            version: to.version,
            created_at: to.created_at,
            updated_at: to.updated_at,
            slug: to.slug,
            title: to.title,
            description: to.description,
            visibility: to.visibility,
        }
    }
}
impl From<PostCollection> for PostCollectionTO {
    fn from(entity: PostCollection) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            slug: entity.slug,
            title: entity.title,
            description: entity.description,
            visibility: entity.visibility,
        }
    }
}
impl From<PostCollectionInfo> for PostCollectionInfoTO {
    fn from(entity: PostCollectionInfo) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            slug: entity.slug,
            title: entity.title,
            description: entity.description,
            visibility: entity.visibility,
        }
    }
}

// ========== Table endpoints ==========
#[get("")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .post_collection_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        PostCollectionTO::from,
    )
}

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(
        state
            .post_collection_service
            .count(query.to_filters())
            .await,
    )
}

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_collection_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostCollectionTO::from),
    )
}

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_collection_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostCollectionTO::from),
    )
}

#[post("")]
pub async fn create(state: Data<AppState>, data: Json<PostCollectionCreateTO>) -> impl Responder {
    respond_result(
        state
            .post_collection_service
            .create(&PostCollectionCreate::from(data.into_inner()))
            .await
            .map(PostCollectionTO::from),
    )
}

#[put("/{id}")]
pub async fn update(state: Data<AppState>, data: Json<PostCollectionTO>) -> impl Responder {
    respond_result(
        state
            .post_collection_service
            .update(&PostCollection::from(data.into_inner()))
            .await
            .map(PostCollectionTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_collection_service
            .delete_by_id(id.into_inner())
            .await,
    )
}

#[delete("/uid/{uid}")]
pub async fn delete_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_collection_service
            .delete_by_uid(uid.into_inner())
            .await,
    )
}

// ========== Info endpoints ==========
#[get("/info")]
pub async fn get_many_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .post_collection_info_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        PostCollectionInfoTO::from,
    )
}

#[get("/info/count")]
pub async fn count_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(
        state
            .post_collection_info_service
            .count(query.to_filters())
            .await,
    )
}

#[get("/{id}/info")]
pub async fn get_info_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_collection_info_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostCollectionInfoTO::from),
    )
}

#[get("/uid/{uid}/info")]
pub async fn get_info_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_collection_info_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostCollectionInfoTO::from),
    )
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/post_collections")
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

impl From<PostCollectionCreateTO> for PostCollectionCreate {
    fn from(to: PostCollectionCreateTO) -> Self {
        Self {
            slug: to.slug,
            title: to.title,
            visibility: to.visibility,
        }
    }
}
