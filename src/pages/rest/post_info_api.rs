use crate::business::post_service::PostInfo;
use crate::define_readonly_to_with_common_fields_fe;
use leptos::prelude::ServerFnError;
use leptos::*;

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

    // Extract app state
    let state: Data<AppState> = extract().await?;
    let query_options = QueryOptions {
        first_result: Some(first_result as i32),
        max_results: Some(max_results),
        sort,
        p_filters,
        a_filters,
        search,
    };

    // Build sorts and filters using QueryOptions utilities on the server
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

#[server(name=CountPostInfos,prefix="/load", endpoint="/posts/count/info")]
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
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let state: Data<AppState> = extract().await?;
    let result = state.post_info_service.get_by_id(id).await;
    match result {
        Ok(Some(p)) => Ok(PostInfoTO::from(p)),
        Ok(None) => Err(ServerFnError::ServerError(
            crate::business::error::CoreError::not_found("error.post_not_found").to_json(),
        )),
        Err(e) => Err(ServerFnError::ServerError(e.to_json())),
    }
}
