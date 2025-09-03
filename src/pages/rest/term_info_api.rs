use crate::business::taxonomy_service::TermInfo;
use crate::define_readonly_to_with_common_fields_fe;
use leptos::prelude::ServerFnError;
use leptos::*;

define_readonly_to_with_common_fields_fe!(TermInfo {
    pub taxonomy_id: i32,
    pub taxonomy_code: String,
    pub taxonomy_name: String,
    pub parent_id: Option<i32>,
    pub parent_slug: Option<String>,
    pub parent_name: Option<String>,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
});

impl From<TermInfo> for TermInfoTO {
    fn from(entity: TermInfo) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            taxonomy_id: entity.taxonomy_id,
            taxonomy_code: entity.taxonomy_code,
            taxonomy_name: entity.taxonomy_name,
            parent_id: entity.parent_id,
            parent_slug: entity.parent_slug,
            parent_name: entity.parent_name,
            slug: entity.slug,
            name: entity.name,
            description: entity.description,
        }
    }
}

#[server(name=LoadTermInfos,prefix="/load", endpoint="/terms/info")]
pub async fn load_term_infos(
    first_result: i64,
    max_results: i32,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
) -> Result<Vec<TermInfoTO>, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    let query_options = QueryOptions { first_result: Some(first_result as i32), max_results: Some(max_results), sort, p_filters, a_filters, search };

    state
        .term_info_service
        .get_many(
            query_options.to_sort_criteria(),
            query_options.first_result,
            query_options.max_results,
            query_options.to_filters(),
        )
        .await
        .map(|items| items.into_iter().map(TermInfoTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountTermInfos,prefix="/load", endpoint="/terms/count/info")]
pub async fn count_term_infos(
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
) -> Result<i64, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use leptos_actix::extract;

    let state: actix_web::web::Data<AppState> = extract().await?;
    let query_options = QueryOptions { first_result: None, max_results: None, sort: Some("-updated_at".to_string()), p_filters, a_filters, search };

    state
        .term_info_service
        .count(query_options.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}
