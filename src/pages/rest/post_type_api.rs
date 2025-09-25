use crate::business::post_type_service::{PostType, PostTypeCreate, PostTypeInfo};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_fe, define_to_with_common_fields_fe};
use leptos::prelude::ServerFnError;
use leptos::*;

// Table TO
define_to_with_common_fields_fe!(PostType { pub code: String, pub name: String, });
// View TO
define_readonly_to_with_common_fields_fe!(PostTypeInfo { pub code: String, pub name: String, });

impl From<PostType> for PostTypeTO {
    fn from(e: PostType) -> Self {
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
impl From<PostTypeInfo> for PostTypeInfoTO {
    fn from(e: PostTypeInfo) -> Self {
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

#[server(name=LoadPostTypes, prefix="/load", endpoint="/post_types")]
pub async fn load_post_types(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<PostTypeTO>, ServerFnError> {
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
        .post_type_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(PostTypeTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountPostTypes, prefix="/load", endpoint="/post_types/count")]
pub async fn count_post_types(
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
        .post_type_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadPostTypeById, prefix="/load", endpoint="/post_types/get")]
pub async fn load_post_type_by_id(id: i32) -> Result<PostTypeTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_type_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostTypeTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadPostTypeByUid, prefix="/load", endpoint="/post_types/get-uid")]
pub async fn load_post_type_by_uid(uid: String) -> Result<PostTypeTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_type_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostTypeTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CreatePostType, prefix="/load", endpoint="/post_types/create")]
pub async fn create_post_type(code: String, name: String) -> Result<PostTypeTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let c = PostTypeCreate { code, name };
    state
        .post_type_service
        .create(&c)
        .await
        .map(PostTypeTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=UpdatePostType, prefix="/load", endpoint="/post_types/update")]
pub async fn update_post_type(entity: PostTypeTO) -> Result<PostTypeTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let e: PostType = PostType {
        id: entity.id,
        uid: entity.uid,
        version: entity.version,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
        code: entity.code,
        name: entity.name,
    };
    state
        .post_type_service
        .update(&e)
        .await
        .map(PostTypeTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeletePostTypeById, prefix="/load", endpoint="/post_types/delete")]
pub async fn delete_post_type_by_id(id: i32) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_type_service
        .delete_by_id(id)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeletePostTypeByUid, prefix="/load", endpoint="/post_types/delete-uid")]
pub async fn delete_post_type_by_uid(uid: String) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_type_service
        .delete_by_uid(uid)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

// Info
#[server(name=LoadPostTypeInfos, prefix="/load", endpoint="/post_types/info")]
pub async fn load_post_type_infos(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<PostTypeInfoTO>, ServerFnError> {
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
        .post_type_info_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(PostTypeInfoTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountPostTypeInfos, prefix="/load", endpoint="/post_types/info/count")]
pub async fn count_post_type_infos(
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
        .post_type_info_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadPostTypeInfoById, prefix="/load", endpoint="/post_types/id/info")]
pub async fn load_post_type_info_by_id(id: i32) -> Result<PostTypeInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_type_info_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostTypeInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadPostTypeInfoByUid, prefix="/load", endpoint="/post_types/uid/info")]
pub async fn load_post_type_info_by_uid(uid: String) -> Result<PostTypeInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_type_info_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostTypeInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}
