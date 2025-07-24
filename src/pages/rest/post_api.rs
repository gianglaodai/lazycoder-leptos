use crate::business::post_service::{Post, PostStatus};
use crate::business::sort::SortCriterion;
use crate::define_to_with_common_fields_fe;
use leptos::prelude::ServerFnError;
use leptos::*;
use std::str::FromStr;

define_to_with_common_fields_fe!(PostTO {
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
#[server]
pub async fn load_posts(first_result: i64, max_results: i32) -> Result<Vec<PostTO>, ServerFnError> {
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;

    let result = state
        .post_service
        .get_many(
            vec![SortCriterion {
                field: "updated_at".to_owned(),
                ascending: false,
            }],
            Some(first_result as i32),
            Some(max_results),
            vec![],
        )
        .await
        .map(|posts| posts.into_iter().map(PostTO::from).collect::<Vec<PostTO>>())
        .map_err(|e| ServerFnError::ServerError(e.to_string()));
    result
}

#[server]
pub async fn count_posts() -> Result<i64, ServerFnError> {
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    state
        .post_service
        .count(vec![])
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
