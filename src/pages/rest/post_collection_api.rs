use crate::business::post_collection_service::{
    PostCollection, PostCollectionCreate, PostCollectionInfo,
};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_fe, define_to_with_common_fields_fe};
use leptos::prelude::ServerFnError;
use leptos::*;

// Table TO
define_to_with_common_fields_fe!(PostCollection {
    pub slug: String,
    pub title: String,
    pub visibility: String,
    pub description: Option<String>,
});
// View TO
define_readonly_to_with_common_fields_fe!(PostCollectionInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub visibility: String,
});

impl From<PostCollection> for PostCollectionTO {
    fn from(e: PostCollection) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            slug: e.slug,
            title: e.title,
            visibility: e.visibility,
            description: e.description,
        }
    }
}
impl From<PostCollectionTO> for PostCollection {
    fn from(t: PostCollectionTO) -> Self {
        Self {
            id: t.id,
            uid: t.uid,
            version: t.version,
            created_at: t.created_at,
            updated_at: t.updated_at,
            slug: t.slug,
            title: t.title,
            visibility: t.visibility,
            description: t.description,
        }
    }
}
impl From<PostCollectionInfo> for PostCollectionInfoTO {
    fn from(e: PostCollectionInfo) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            slug: e.slug,
            title: e.title,
            description: e.description,
            visibility: e.visibility,
        }
    }
}

// Table
#[server(name=LoadPostCollections, prefix="/load", endpoint="/post_collections")]
pub async fn load_post_collections(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<PostCollectionTO>, ServerFnError> {
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
        .post_collection_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(PostCollectionTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountPostCollections, prefix="/load", endpoint="/post_collections/count")]
pub async fn count_post_collections(
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
        .post_collection_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=GetPostCollectionById, prefix="/load", endpoint="/post_collections/get")]
pub async fn get_post_collection_by_id(id: i32) -> Result<PostCollectionTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_collection_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostCollectionTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=GetPostCollectionByUid, prefix="/load", endpoint="/post_collections/get-uid")]
pub async fn get_post_collection_by_uid(uid: String) -> Result<PostCollectionTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_collection_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostCollectionTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CreatePostCollection, prefix="/load", endpoint="/post_collections/create")]
pub async fn create_post_collection(
    slug: String,
    title: String,
    visibility: String,
) -> Result<PostCollectionTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let c = PostCollectionCreate {
        slug,
        title,
        visibility,
    };
    state
        .post_collection_service
        .create(&c)
        .await
        .map(PostCollectionTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=UpdatePostCollection, prefix="/load", endpoint="/post_collections/update")]
pub async fn update_post_collection(
    entity: PostCollectionTO,
) -> Result<PostCollectionTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let e: PostCollection = entity.into();
    state
        .post_collection_service
        .update(&e)
        .await
        .map(PostCollectionTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeletePostCollectionById, prefix="/load", endpoint="/post_collections/delete")]
pub async fn delete_post_collection_by_id(id: i32) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_collection_service
        .delete_by_id(id)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeletePostCollectionByUid, prefix="/load", endpoint="/post_collections/delete-uid")]
pub async fn delete_post_collection_by_uid(uid: String) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_collection_service
        .delete_by_uid(uid)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

// Info
#[server(name=LoadPostCollectionInfos, prefix="/load", endpoint="/post_collections/info")]
pub async fn load_post_collection_infos(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<PostCollectionInfoTO>, ServerFnError> {
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
        .post_collection_info_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(PostCollectionInfoTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountPostCollectionInfos, prefix="/load", endpoint="/post_collections/info/count")]
pub async fn count_post_collection_infos(
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
        .post_collection_info_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=GetPostCollectionInfoById, prefix="/load", endpoint="/post_collections/id/info")]
pub async fn get_post_collection_info_by_id(
    id: i32,
) -> Result<PostCollectionInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_collection_info_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostCollectionInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=GetPostCollectionInfoByUid, prefix="/load", endpoint="/post_collections/uid/info")]
pub async fn get_post_collection_info_by_uid(
    uid: String,
) -> Result<PostCollectionInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .post_collection_info_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(PostCollectionInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}
