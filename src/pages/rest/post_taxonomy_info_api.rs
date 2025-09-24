use crate::business::post_taxonomy_service::PostTaxonomyInfo;
use crate::define_readonly_to_with_common_fields_fe;
use leptos::prelude::ServerFnError;
use leptos::*;
use crate::common::service::ViewService;

define_readonly_to_with_common_fields_fe!(PostTaxonomyInfo {
    pub code: String,
    pub name: String,
});

impl From<PostTaxonomyInfo> for PostTaxonomyInfoTO {
    fn from(entity: PostTaxonomyInfo) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            code: entity.code,
            name: entity.name,
        }
    }
}

#[server(name=LoadPostTaxonomyInfos,prefix="/load", endpoint="/post_taxonomies/info")]
pub async fn load_post_taxonomy_infos(
    first_result: i64,
    max_results: i32,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
) -> Result<Vec<PostTaxonomyInfoTO>, ServerFnError> {
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

    state
        .post_taxonomy_info_service
        .get_many(
            query_options.to_sort_criteria(),
            query_options.first_result,
            query_options.max_results,
            query_options.to_filters(),
        )
        .await
        .map(|items| items.into_iter().map(PostTaxonomyInfoTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountPostTaxonomyInfos,prefix="/load", endpoint="/post_taxonomies/count/info")]
pub async fn count_post_taxonomy_infos(
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
        .post_taxonomy_info_service
        .count(query_options.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}
