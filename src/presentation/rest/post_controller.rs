use crate::business::post_service::{Post, PostCreate, PostStatus};
use crate::define_to_with_common_fields_be;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};
use std::str::FromStr;

define_to_with_common_fields_be!(Post {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: String,
    pub user_id: i32,
    pub type_id: i32,
});

impl From<PostTO> for Post {
    fn from(to: PostTO) -> Self {
        Self {
            id: to.id,
            uid: to.uid,
            version: to.version,
            created_at: to.created_at,
            updated_at: to.updated_at,
            slug: to.slug,
            title: to.title,
            summary: to.summary,
            content: to.content,
            status: PostStatus::from_str(&to.status).unwrap_or(PostStatus::DRAFT),
            user_id: to.user_id,
            type_id: to.type_id,
        }
    }
}

impl From<PostCreateTO> for PostCreate {
    fn from(to: PostCreateTO) -> Self {
        Self {
            slug: to.slug,
            title: to.title,
            summary: to.summary,
            content: to.content,
            status: PostStatus::from_str(&to.status).unwrap_or(PostStatus::DRAFT),
            user_id: to.user_id,
            type_id: to.type_id,
        }
    }
}

impl From<Post> for PostTO {
    fn from(entity: Post) -> Self {
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
            type_id: entity.type_id,
        }
    }
}

#[get("")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .post_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        PostTO::from,
    )
}

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.post_service.count(query.to_filters()).await)
}

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .post_service
            .get_by_id(id.into_inner())
            .await
            .map(|post| post.unwrap())
            .map(PostTO::from),
    )
}

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .post_service
            .get_by_uid(uid.into_inner())
            .await
            .map(|post| post.unwrap())
            .map(PostTO::from),
    )
}

#[post("")]
pub async fn create(state: Data<AppState>, post: Json<PostCreateTO>) -> impl Responder {
    respond_result(
        state
            .post_service
            .create(&PostCreate::from(post.into_inner()))
            .await
            .map(PostTO::from),
    )
}

#[put("/{id}")]
pub async fn update(
    state: Data<AppState>,
    id: Path<i32>,
    mut post: Json<PostTO>,
) -> impl Responder {
    post.id = id.into_inner();
    respond_result(
        state
            .post_service
            .update(&Post::from(post.into_inner()))
            .await
            .map(PostTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(state.post_service.delete_by_id(id.into_inner()).await)
}

#[delete("/uid/{uid}")]
pub async fn delete_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(state.post_service.delete_by_uid(uid.into_inner()).await)
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/posts")
            .service(get_many)
            .service(count)
            .service(get_by_id)
            .service(get_by_uid)
            .service(create)
            .service(update)
            .service(delete_by_id)
            .service(delete_by_uid),
    );
}
