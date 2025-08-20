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

#[server(name=LoadPostInfos,prefix="/load", endpoint="/posts-info")]
pub async fn load_post_infos(first_result: i64, max_results: i32) -> Result<Vec<PostInfoTO>, ServerFnError> {
    use crate::business::sort::SortCriterion;
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    let result = state
        .post_info_service
        .get_many(
            vec![SortCriterion { field: "updated_at".to_owned(), ascending: false }],
            Some(first_result as i32),
            Some(max_results),
            vec![],
        )
        .await
        .map(|items| items.into_iter().map(PostInfoTO::from).collect::<Vec<PostInfoTO>>())
        .map_err(|e| ServerFnError::ServerError(e.to_string()));
    result
}

#[server(name=CountPostInfos,prefix="/load", endpoint="/posts-info/count")]
pub async fn count_post_infos() -> Result<i64, ServerFnError> {
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    state
        .post_info_service
        .count(vec![])
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[server(name=LoadPostInfoById, prefix="/load", endpoint="/posts-info/get")]
pub async fn load_post_info_by_id(id: i32) -> Result<PostInfoTO, ServerFnError> {
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    let result = state.post_info_service.get_by_id(id).await;
    match result {
        Ok(Some(p)) => Ok(PostInfoTO::from(p)),
        Ok(None) => Err(ServerFnError::ServerError("Not Found".to_string())),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
