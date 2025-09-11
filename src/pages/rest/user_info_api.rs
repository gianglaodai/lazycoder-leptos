use crate::define_readonly_to_with_common_fields_fe;
use leptos::prelude::ServerFnError;
use leptos::*;

// Mirror of minimal user info for admin listing
define_readonly_to_with_common_fields_fe!(UserInfo {
    pub username: String,
    pub email: String,
    pub role: String,
});

#[server(name=LoadUserInfos,prefix="/load", endpoint="/users/info")]
pub async fn load_user_infos(
    first_result: i64,
    max_results: i32,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
) -> Result<Vec<UserInfoTO>, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use leptos_actix::extract;
    use actix_web::web::Data;

    let state: Data<AppState> = extract().await?;
    let query_options = QueryOptions {
        first_result: Some(first_result as i32),
        max_results: Some(max_results),
        sort,
        p_filters,
        a_filters,
        search,
    };

    state
        .user_service
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
                .map(|u| UserInfoTO {
                    id: u.id,
                    uid: u.uid,
                    version: u.version,
                    created_at: u.created_at,
                    updated_at: u.updated_at,
                    username: u.username,
                    email: u.email,
                    role: u.role.as_str().to_string(),
                })
                .collect()
        })
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountUserInfos,prefix="/load", endpoint="/users/count/info")]
pub async fn count_user_infos(
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
) -> Result<i64, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use leptos_actix::extract;
    use actix_web::web::Data;

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
        .user_service
        .count(query_options.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}
