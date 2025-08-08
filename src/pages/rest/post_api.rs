use crate::business::post_service::{Post, PostStatus};
use crate::define_to_with_common_fields_fe;
use leptos::prelude::ServerFnError;
use leptos::*;
use std::str::FromStr;

define_to_with_common_fields_fe!(Post {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: String,
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
            author_id: None,
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
        }
    }
}
#[server(name=LoadPosts,prefix="/load", endpoint="/posts")]
pub async fn load_posts(first_result: i64, max_results: i32) -> Result<Vec<PostTO>, ServerFnError> {
    use crate::business::sort::SortCriterion;
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

#[server(name=CountPosts,prefix="/load", endpoint="/posts/count")]
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
