use crate::business::error::CoreError;
use crate::business::post_service::{Post, PostStatus};
use crate::define_to_with_common_fields_fe;
use leptos::prelude::ServerFnError;
use leptos::*;
use std::str::FromStr;
use crate::pages::rest::post_info_api::PostInfoTO;

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
            user_id: -1,
            type_id: -1,
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
pub async fn load_posts(first_result: i64, max_results: i32) -> Result<Vec<PostInfoTO>, ServerFnError> {
    use crate::business::sort::SortCriterion;
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    let result = state
        .post_info_service
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
        .map(|posts| posts.into_iter().map(PostInfoTO::from).collect::<Vec<PostInfoTO>>())
        .map_err(|e| ServerFnError::ServerError(e.to_json()));
    result
}
#[server(name=CountPosts,prefix="/load", endpoint="/posts/count")]
pub async fn count_posts() -> Result<i64, ServerFnError> {
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    state
        .post_info_service
        .count(vec![])
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

fn slugify(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_dash = false;
    for ch in input.chars() {
        let c = ch.to_ascii_lowercase();
        if c.is_ascii_alphanumeric() {
            out.push(c);
            last_dash = false;
        } else if c.is_ascii_whitespace() || c == '-' || c == '_' {
            if !last_dash && !out.is_empty() {
                out.push('-');
                last_dash = true;
            }
        } else {
            // non-ascii or punctuation -> treat as separator
            if !last_dash && !out.is_empty() {
                out.push('-');
                last_dash = true;
            }
        }
    }
    // trim trailing '-'
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        "post".to_string()
    } else {
        out
    }
}
#[server(name=CreatePost, prefix="/load", endpoint="/posts/create")]
pub async fn create_post(title: String, type_id: i32, user_id: i32) -> Result<PostTO, ServerFnError> {
    use crate::business::post_service::{PostCreate, PostStatus};
    use crate::state::AppState;
    use actix_session::SessionExt as _;
    use leptos_actix::extract;

    // Guard: require ADMIN role from server session
    let req: actix_web::HttpRequest = extract().await?;
    let session = req.get_session();
    let role: Option<String> = match session.get("role") {
        Ok(v) => v,
        Err(_) => {
            return Err(ServerFnError::ServerError(
                CoreError::unauthorized("error.missing_session").to_json(),
            ))
        }
    };
    match role.as_deref() {
        Some("ADMIN") => {}
        _ => {
            return Err(ServerFnError::ServerError(
                CoreError::forbidden("error.forbidden").to_json(),
            ))
        }
    }

    let state: actix_web::web::Data<AppState> = extract().await?;
    let create = PostCreate {
        slug: slugify(&title),
        title,
        summary: "".to_string(),
        content: "".to_string(),
        status: PostStatus::DRAFT,
        user_id,
        type_id,
    };
    state
        .post_service
        .create(&create)
        .await
        .map(PostTO::from)
        .map_err(|e| e.to_json())
        .map_err(ServerFnError::ServerError)
}

#[server(name=UpdatePost, prefix="/load", endpoint="/posts/update")]
pub async fn update_post(post: PostTO) -> Result<PostTO, ServerFnError> {
    use crate::state::AppState;
    use actix_session::SessionExt as _;
    use leptos_actix::extract;

    // Guard: require ADMIN role from server session
    let req: actix_web::HttpRequest = extract().await?;
    let session = req.get_session();
    let role: Option<String> = match session.get("role") {
        Ok(v) => v,
        Err(_) => {
            return Err(ServerFnError::ServerError(
                CoreError::unauthorized("error.missing_session").to_json(),
            ))
        }
    };
    match role.as_deref() {
        Some("ADMIN") => {}
        _ => {
            return Err(ServerFnError::ServerError(
                CoreError::forbidden("error.forbidden").to_json(),
            ))
        }
    }

    let state: actix_web::web::Data<AppState> = extract().await?;
    let entity: Post = post.into();
    state
        .post_service
        .update(&entity)
        .await
        .map(PostTO::from)
        .map_err(|e| e.to_json())
        .map_err(ServerFnError::ServerError)
}

#[server(name=DeletePost, prefix="/load", endpoint="/posts/delete")]
pub async fn delete_post(id: i32) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_session::SessionExt as _;
    use leptos_actix::extract;

    // Guard: require ADMIN role from server session
    let req: actix_web::HttpRequest = extract().await?;
    let session = req.get_session();
    let role: Option<String> = match session.get("role") {
        Ok(v) => v,
        Err(_) => {
            return Err(ServerFnError::ServerError(
                CoreError::unauthorized("error.missing_session").to_json(),
            ))
        }
    };
    match role.as_deref() {
        Some("ADMIN") => {}
        _ => {
            return Err(ServerFnError::ServerError(
                CoreError::forbidden("error.forbidden").to_json(),
            ))
        }
    }

    let state: actix_web::web::Data<AppState> = extract().await?;
    state
        .post_service
        .delete_by_id(id)
        .await
        .map_err(|e| e.to_json())
        .map_err(ServerFnError::ServerError)
}

#[server(name=LoadPostById, prefix="/load", endpoint="/posts/get")]
pub async fn load_post_by_id(id: i32) -> Result<PostTO, ServerFnError> {
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    let result = state.post_service.get_by_id(id).await;
    match result {
        Ok(Some(p)) => Ok(PostTO::from(p)),
        Ok(None) => Err(ServerFnError::ServerError(
            CoreError::not_found("error.post_not_found").to_json(),
        )),
        Err(e) => Err(ServerFnError::ServerError(e.to_json())),
    }
}
