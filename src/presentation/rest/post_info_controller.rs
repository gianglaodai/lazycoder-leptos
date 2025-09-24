use crate::business::post_service::PostInfo;
use crate::define_readonly_to_with_common_fields_be;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Path, Query, ServiceConfig};
use actix_web::{get, Responder};
use crate::common::error::CoreError;
use crate::common::service::ViewService;

define_readonly_to_with_common_fields_be!(PostInfo {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: String,
    pub user_id: i32,
    pub username: String,
    pub email: String,
});

impl From<PostInfo> for PostInfoTO {
    fn from(entity: PostInfo) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            slug: entity.slug,
            title: entity.title,
            summary: entity.summary,
            content: entity.content,
            status: entity.status.as_str().to_string(),
            user_id: entity.user_id,
            username: entity.username,
            email: entity.email,
        }
    }
}

#[get("/info")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .post_info_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        PostInfoTO::from,
    )
}

#[get("/info/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.post_info_service.count(query.to_filters()).await)
}

#[get("/{id}/info")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_info_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostInfoTO::from),
    )
}

#[get("/uid/{uid}/info")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_info_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(PostInfoTO::from),
    )
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/posts")
            .service(get_many)
            .service(count)
            .service(get_by_id)
            .service(get_by_uid),
    );
}
