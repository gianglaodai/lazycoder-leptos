use crate::business::post_collection_service::PostCollectionInfo;
use crate::define_readonly_to_with_common_fields_be;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Path, Query, ServiceConfig};
use actix_web::{get, Responder};

define_readonly_to_with_common_fields_be!(PostCollectionInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub visibility: String,
});

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

#[get("/info")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
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
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(
        state
            .post_collection_info_service
            .count(query.to_filters())
            .await,
    )
}

#[get("/{id}/info")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_collection_info_service
            .get_by_id(id.into_inner())
            .await
            .map(|it| it.unwrap())
            .map(PostCollectionInfoTO::from),
    )
}

#[get("/uid/{uid}/info")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_collection_info_service
            .get_by_uid(uid.into_inner())
            .await
            .map(|it| it.unwrap())
            .map(PostCollectionInfoTO::from),
    )
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/post_collections")
            .service(get_many)
            .service(count)
            .service(get_by_id)
            .service(get_by_uid),
    );
}
