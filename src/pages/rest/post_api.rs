use crate::business::post_service::{Post, PostInfo, PostStatus};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_fe, define_to_with_common_fields_fe};
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

define_readonly_to_with_common_fields_fe!(PostInfo {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: String,
    pub user_id: i32,
    pub username: String,
    pub email: String,
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

#[server(name=LoadPosts,prefix="/load", endpoint="/posts")]
pub async fn load_posts(
    first_result: i64,
    max_results: i32,
) -> Result<Vec<PostInfoTO>, ServerFnError> {
    use crate::common::sort::SortCriterion;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let state: Data<AppState> = extract().await?;
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
        .map(|posts| {
            posts
                .into_iter()
                .map(PostInfoTO::from)
                .collect::<Vec<PostInfoTO>>()
        })
        .map_err(|e| ServerFnError::ServerError(e.to_json()));
    result
}
#[server(name=CountPosts,prefix="/load", endpoint="/posts/count")]
pub async fn count_posts() -> Result<i64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let state: Data<AppState> = extract().await?;
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
pub async fn create_post(title: String, type_id: i32) -> Result<PostTO, ServerFnError> {
    use crate::business::post_service::PostCreate;
    use crate::state::AppState;
    use actix_session::SessionExt as _;
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use leptos_actix::extract;

    // Guard: require ADMIN role from server session
    let req: HttpRequest = extract().await?;
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

    // Get current user_id from session
    let user: Option<crate::pages::rest::auth_api::UserTO> = match session.get("user") {
        Ok(v) => v,
        Err(_) => {
            return Err(ServerFnError::ServerError(
                CoreError::unauthorized("error.missing_session").to_json(),
            ))
        }
    };
    let user_id = match user {
        Some(u) => u.id,
        None => {
            return Err(ServerFnError::ServerError(
                CoreError::unauthorized("error.unauthorized").to_json(),
            ))
        }
    };

    let state: Data<AppState> = extract().await?;
    let create = PostCreate {
        title,
        type_id,
        user_id,
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
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use leptos_actix::extract;

    let req: HttpRequest = extract().await?;
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

    let state: Data<AppState> = extract().await?;
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
    use actix_web::web::Data;
    use actix_web::HttpRequest;
    use leptos_actix::extract;

    // Guard: require ADMIN role from server session
    let req: HttpRequest = extract().await?;
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

    let state: Data<AppState> = extract().await?;
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
    use actix_web::web::Data;
    use leptos_actix::extract;

    let state: Data<AppState> = extract().await?;
    let result = state.post_service.get_by_id(id).await;
    match result {
        Ok(Some(p)) => Ok(PostTO::from(p)),
        Ok(None) => Err(ServerFnError::ServerError(
            CoreError::not_found("error.post_not_found").to_json(),
        )),
        Err(e) => Err(ServerFnError::ServerError(e.to_json())),
    }
}

#[server(name=LoadPostInfos,prefix="/load", endpoint="/posts/info")]
pub async fn load_post_infos(
    first_result: i64,
    max_results: i32,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
) -> Result<Vec<PostInfoTO>, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let state: Data<AppState> = extract().await?;
    let query_options = QueryOptions {
        first_result: Some(first_result as i32),
        max_results: Some(max_results),
        sort,
        p_filters,
        a_filters,
        search,
    };

    let result = state
        .post_info_service
        .get_many(
            query_options.to_sort_criteria(),
            query_options.first_result,
            query_options.max_results,
            query_options.to_filters(),
        )
        .await
        .map(|items| {
            items
                .into_iter()
                .map(PostInfoTO::from)
                .collect::<Vec<PostInfoTO>>()
        })
        .map_err(|e| ServerFnError::ServerError(e.to_json()));
    result
}

#[server(name=CountPostInfos,prefix="/load", endpoint="/posts/info/count")]
pub async fn count_post_infos(
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
) -> Result<i64, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let state: Data<AppState> = extract().await?;
    let query_options = QueryOptions {
        first_result: None,
        max_results: None,
        sort: Some("-updated_at".to_string()),
        p_filters,
        a_filters,
        search,
    };
    state
        .post_info_service
        .count(query_options.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadPostInfoById, prefix="/load", endpoint="/posts/id/info")]
pub async fn load_post_info_by_id(id: i32) -> Result<PostInfoTO, ServerFnError> {
    use crate::common::error::CoreError;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let state: Data<AppState> = extract().await?;
    let result = state.post_info_service.get_by_id(id).await;
    match result {
        Ok(Some(p)) => Ok(PostInfoTO::from(p)),
        Ok(None) => Err(ServerFnError::ServerError(
            CoreError::not_found("error.post_not_found").to_json(),
        )),
        Err(e) => Err(ServerFnError::ServerError(e.to_json())),
    }
}
