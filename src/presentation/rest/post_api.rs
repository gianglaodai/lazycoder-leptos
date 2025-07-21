use crate::business::post_service::{Post, PostStatus};
use crate::define_to_with_common_fields;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};
use std::str::FromStr;
use uuid::Uuid;

define_to_with_common_fields!(PostTO {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: String,
});

impl From<PostTO> for Post {
    fn from(post: PostTO) -> Self {
        Self {
            id: post.id,
            uid: post.uid,
            created_at: post.created_at,
            updated_at: post.updated_at,
            slug: post.slug,
            title: post.title,
            summary: post.summary,
            content: post.content,
            status: PostStatus::from_str(&post.status).unwrap_or(PostStatus::DRAFT),
            author_id: None,
        }
    }
}

impl From<Post> for PostTO {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            uid: post.uid,
            created_at: post.created_at,
            updated_at: post.updated_at,
            slug: post.slug,
            title: post.title,
            summary: post.summary,
            content: post.content,
            status: post.status.as_str().to_string(),
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

#[post("")]
pub async fn create(state: Data<AppState>, post: Json<PostTO>) -> impl Responder {
    respond_result(
        state
            .post_service
            .create(&Post::from(post.into_inner()))
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
    post.id = Some(id.into_inner());
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
pub async fn delete_by_uid(state: Data<AppState>, uid: Path<Uuid>) -> impl Responder {
    respond_result(state.post_service.delete_by_uid(uid.into_inner()).await)
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/posts")
            .service(get_many)
            .service(count)
            .service(get_by_id)
            .service(create)
            .service(update)
            .service(delete_by_id)
            .service(delete_by_uid),
    );
}
