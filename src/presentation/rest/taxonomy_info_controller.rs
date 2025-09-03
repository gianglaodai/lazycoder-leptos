use crate::business::taxonomy_service::{PostTaxonomyInfo};
use crate::define_readonly_to_with_common_fields_be;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Path, Query, ServiceConfig};
use actix_web::{get, Responder};

define_readonly_to_with_common_fields_be!(PostTaxonomyInfo {
    pub code: String,
    pub name: String,
});

impl From<PostTaxonomyInfo> for PostTaxonomyInfoTO {
    fn from(entity: PostTaxonomyInfo) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            code: entity.code,
            name: entity.name,
        }
    }
}

#[get("")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
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

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(
        state
            .post_taxonomy_info_service
            .count(query.to_filters())
            .await,
    )
}

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_taxonomy_info_service
            .get_by_id(id.into_inner())
            .await
            .map(|it| it.unwrap())
            .map(PostTaxonomyInfoTO::from),
    )
}

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_taxonomy_info_service
            .get_by_uid(uid.into_inner())
            .await
            .map(|it| it.unwrap())
            .map(PostTaxonomyInfoTO::from),
    )
}

pub fn routes_post_taxonomies(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/post_taxonomies")
            .service(get_many)
            .service(count)
            .service(get_by_id)
            .service(get_by_uid),
    );
}

