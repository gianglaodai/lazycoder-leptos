use crate::business::post_taxonomy_service::{PostTaxonomy, PostTaxonomyCreate, PostTaxonomyInfo};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_fe, define_to_with_common_fields_fe};
use leptos::prelude::ServerFnError;
use leptos::*;

// Table TO
define_to_with_common_fields_fe!(PostTaxonomy { pub code: String, pub name: String, });
// View TO
define_readonly_to_with_common_fields_fe!(PostTaxonomyInfo { pub code: String, pub name: String, });

impl From<PostTaxonomy> for PostTaxonomyTO {
    fn from(e: PostTaxonomy) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            code: e.code,
            name: e.name,
        }
    }
}
impl From<PostTaxonomyInfo> for PostTaxonomyInfoTO {
    fn from(e: PostTaxonomyInfo) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            code: e.code,
            name: e.name,
        }
    }
}

#[server(name=LoadPostTaxonomies, prefix="/load", endpoint="/post_taxonomies")]
pub async fn load_post_taxonomies(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<PostTaxonomyTO>, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let q = QueryOptions {
        first_result,
        max_results,
        sort,
        search,
        p_filters,
        a_filters: None,
    };
    state
        .post_taxonomy_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(PostTaxonomyTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountPostTaxonomies, prefix="/load", endpoint="/post_taxonomies/count")]
pub async fn count_post_taxonomies(
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<i64, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let q = QueryOptions {
        first_result: None,
        max_results: None,
        sort: None,
        search,
        p_filters,
        a_filters: None,
    };
    state
        .post_taxonomy_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadPostTaxonomyById, prefix="/load", endpoint="/post_taxonomies/get")]
pub async fn load_post_taxonomy_by_id(id: i32) -> Result<PostTaxonomyTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_taxonomy_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostTaxonomyTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadPostTaxonomyByUid, prefix="/load", endpoint="/post_taxonomies/get-uid")]
pub async fn load_post_taxonomy_by_uid(uid: String) -> Result<PostTaxonomyTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_taxonomy_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostTaxonomyTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CreatePostTaxonomy, prefix="/load", endpoint="/post_taxonomies/create")]
pub async fn create_post_taxonomy(
    code: String,
    name: String,
) -> Result<PostTaxonomyTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let c = PostTaxonomyCreate { code, name };
    state
        .post_taxonomy_service
        .create(&c)
        .await
        .map(PostTaxonomyTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=UpdatePostTaxonomy, prefix="/load", endpoint="/post_taxonomies/update")]
pub async fn update_post_taxonomy(entity: PostTaxonomyTO) -> Result<PostTaxonomyTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let e: PostTaxonomy = PostTaxonomy {
        id: entity.id,
        uid: entity.uid,
        version: entity.version,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
        code: entity.code,
        name: entity.name,
    };
    state
        .post_taxonomy_service
        .update(&e)
        .await
        .map(PostTaxonomyTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeletePostTaxonomyById, prefix="/load", endpoint="/post_taxonomies/delete")]
pub async fn delete_post_taxonomy_by_id(id: i32) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_taxonomy_service
        .delete_by_id(id)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeletePostTaxonomyByUid, prefix="/load", endpoint="/post_taxonomies/delete-uid")]
pub async fn delete_post_taxonomy_by_uid(uid: String) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_taxonomy_service
        .delete_by_uid(uid)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

// Info
#[server(name=LoadPostTaxonomyInfos, prefix="/load", endpoint="/post_taxonomies/info")]
pub async fn load_post_taxonomy_infos(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<PostTaxonomyInfoTO>, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let q = QueryOptions {
        first_result,
        max_results,
        sort,
        search,
        p_filters,
        a_filters: None,
    };
    state
        .post_taxonomy_info_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(PostTaxonomyInfoTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountPostTaxonomyInfos, prefix="/load", endpoint="/post_taxonomies/info/count")]
pub async fn count_post_taxonomy_infos(
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<i64, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let q = QueryOptions {
        first_result: None,
        max_results: None,
        sort: None,
        search,
        p_filters,
        a_filters: None,
    };
    state
        .post_taxonomy_info_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadPostTaxonomyInfoById, prefix="/load", endpoint="/post_taxonomies/id/info")]
pub async fn load_post_taxonomy_info_by_id(id: i32) -> Result<PostTaxonomyInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_taxonomy_info_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostTaxonomyInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadPostTaxonomyInfoByUid, prefix="/load", endpoint="/post_taxonomies/uid/info")]
pub async fn load_post_taxonomy_info_by_uid(
    uid: String,
) -> Result<PostTaxonomyInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_taxonomy_info_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostTaxonomyInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}
