use crate::business::post_term_service::PostTermInfo;
use crate::define_readonly_to_with_common_fields_be;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Path, Query, ServiceConfig};
use actix_web::{get, Responder};

define_readonly_to_with_common_fields_be!(PostTermInfo {
    pub post_id: i32,
    pub term_id: i32,
    pub post_slug: Option<String>,
    pub post_title: Option<String>,
    pub term_slug: Option<String>,
    pub term_name: Option<String>,
    pub taxonomy_id: Option<i32>,
    pub taxonomy_code: Option<String>,
});

impl From<PostTermInfo> for PostTermInfoTO {
    fn from(entity: PostTermInfo) -> Self { Self { id: entity.id, uid: entity.uid, version: entity.version, created_at: entity.created_at, updated_at: entity.updated_at, post_id: entity.post_id, term_id: entity.term_id, post_slug: entity.post_slug, post_title: entity.post_title, term_slug: entity.term_slug, term_name: entity.term_name, taxonomy_id: entity.taxonomy_id, taxonomy_code: entity.taxonomy_code } }
}

#[get("")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder { respond_results(state.post_term_info_service.get_many(query.to_sort_criteria(), query.first_result, query.max_results, query.to_filters()).await, PostTermInfoTO::from) }

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder { respond_result(state.post_term_info_service.count(query.to_filters()).await) }

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder { respond_result(state.post_term_info_service.get_by_id(id.into_inner()).await.map(|it| it.unwrap()).map(PostTermInfoTO::from)) }

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder { respond_result(state.post_term_info_service.get_by_uid(uid.into_inner()).await.map(|it| it.unwrap()).map(PostTermInfoTO::from)) }

pub fn routes(cfg: &mut ServiceConfig) { cfg.service(scope("/api/post-terms").service(get_many).service(count).service(get_by_id).service(get_by_uid)); }
