use crate::business::collection_item_service::PostCollectionItemInfo;
use crate::define_readonly_to_with_common_fields_be;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Path, Query, ServiceConfig};
use actix_web::{get, Responder};

define_readonly_to_with_common_fields_be!(PostCollectionItemInfo {
    pub post_collection_id: i32,
    pub post_id: i32,
    pub position: i32,
    pub headline: Option<String>,
    pub collection_slug: Option<String>,
    pub collection_title: Option<String>,
    pub post_slug: Option<String>,
    pub post_title: Option<String>,
});

impl From<PostCollectionItemInfo> for PostCollectionItemInfoTO {
    fn from(entity: PostCollectionItemInfo) -> Self {
        Self { id: entity.id, uid: entity.uid, version: entity.version, created_at: entity.created_at, updated_at: entity.updated_at, post_collection_id: entity.post_collection_id, post_id: entity.post_id, position: entity.position, headline: entity.headline, collection_slug: entity.collection_slug, collection_title: entity.collection_title, post_slug: entity.post_slug, post_title: entity.post_title }
    }
}

#[get("")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(state.post_collection_item_info_service.get_many(query.to_sort_criteria(), query.first_result, query.max_results, query.to_filters()).await, PostCollectionItemInfoTO::from)
}

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder { respond_result(state.post_collection_item_info_service.count(query.to_filters()).await) }

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder { respond_result(state.post_collection_item_info_service.get_by_id(id.into_inner()).await.map(|it| it.unwrap()).map(PostCollectionItemInfoTO::from)) }

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder { respond_result(state.post_collection_item_info_service.get_by_uid(uid.into_inner()).await.map(|it| it.unwrap()).map(PostCollectionItemInfoTO::from)) }

pub fn routes(cfg: &mut ServiceConfig) { cfg.service(scope("/api/post-collection-items").service(get_many).service(count).service(get_by_id).service(get_by_uid)); }
