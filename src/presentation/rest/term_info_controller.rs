use crate::business::term_service::TermInfo;
use crate::define_readonly_to_with_common_fields_be;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Path, Query, ServiceConfig};
use actix_web::{get, Responder};
use crate::common::service::ViewService;

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

impl From<TermInfo> for TermInfoTO {
    fn from(entity: TermInfo) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            taxonomy_id: entity.taxonomy_id,
            taxonomy_code: entity.taxonomy_code,
            taxonomy_name: entity.taxonomy_name,
            parent_id: entity.parent_id,
            parent_slug: entity.parent_slug,
            parent_name: entity.parent_name,
            slug: entity.slug,
            name: entity.name,
            description: entity.description,
        }
    }
}

#[get("/info")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
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
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.term_info_service.count(query.to_filters()).await)
}

#[get("/{id}/info")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .term_info_service
            .get_by_id(id.into_inner())
            .await
            .map(|it| it.unwrap())
            .map(TermInfoTO::from),
    )
}

#[get("/uid/{uid}/info")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .term_info_service
            .get_by_uid(uid.into_inner())
            .await
            .map(|it| it.unwrap())
            .map(TermInfoTO::from),
    )
}

pub fn routes_terms(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/terms")
            .service(get_many)
            .service(count)
            .service(get_by_id)
            .service(get_by_uid),
    );
}
