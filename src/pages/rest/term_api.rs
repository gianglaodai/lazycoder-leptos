use crate::business::term_service::{Term, TermCreate, TermInfo};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_fe, define_to_with_common_fields_fe};
use leptos::prelude::ServerFnError;
use leptos::*;

// Table
define_to_with_common_fields_fe!(Term {
    pub taxonomy_id: i32,
    pub slug: String,
    pub name: String,
    pub parent_id: Option<i32>,
    pub description: Option<String>,
});
// View
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

impl From<Term> for TermTO {
    fn from(e: Term) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            taxonomy_id: e.taxonomy_id,
            slug: e.slug,
            name: e.name,
            parent_id: e.parent_id,
            description: e.description,
        }
    }
}
impl From<TermTO> for Term {
    fn from(t: TermTO) -> Self {
        Self {
            id: t.id,
            uid: t.uid,
            version: t.version,
            created_at: t.created_at,
            updated_at: t.updated_at,
            taxonomy_id: t.taxonomy_id,
            slug: t.slug,
            name: t.name,
            parent_id: t.parent_id,
            description: t.description,
        }
    }
}
impl From<TermInfo> for TermInfoTO {
    fn from(e: TermInfo) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            taxonomy_id: e.taxonomy_id,
            taxonomy_code: e.taxonomy_code,
            taxonomy_name: e.taxonomy_name,
            parent_id: e.parent_id,
            parent_slug: e.parent_slug,
            parent_name: e.parent_name,
            slug: e.slug,
            name: e.name,
            description: e.description,
        }
    }
}

#[server(name=LoadTerms, prefix="/load", endpoint="/terms")]
pub async fn load_terms(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<TermTO>, ServerFnError> {
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
        .term_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(TermTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountTerms, prefix="/load", endpoint="/terms/count")]
pub async fn count_terms(
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<i64, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use leptos_actix::extract;
    let state: actix_web::web::Data<AppState> = extract().await?;
    let q = QueryOptions {
        first_result: None,
        max_results: None,
        sort: None,
        search,
        p_filters,
        a_filters: None,
    };
    state
        .term_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=GetTermById, prefix="/load", endpoint="/terms/get")]
pub async fn get_term_by_id(id: i32) -> Result<TermTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .term_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(TermTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=GetTermByUid, prefix="/load", endpoint="/terms/get-uid")]
pub async fn get_term_by_uid(uid: String) -> Result<TermTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .term_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(TermTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CreateTerm, prefix="/load", endpoint="/terms/create")]
pub async fn create_term(
    taxonomy_id: i32,
    slug: String,
    name: String,
) -> Result<TermTO, ServerFnError> {
    use crate::business::term_service::TermCreate;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let c = TermCreate {
        taxonomy_id,
        slug,
        name,
    };
    state
        .term_service
        .create(&c)
        .await
        .map(TermTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=UpdateTerm, prefix="/load", endpoint="/terms/update")]
pub async fn update_term(entity: TermTO) -> Result<TermTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let e: Term = entity.into();
    state
        .term_service
        .update(&e)
        .await
        .map(TermTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeleteTermById, prefix="/load", endpoint="/terms/delete")]
pub async fn delete_term_by_id(id: i32) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .term_service
        .delete_by_id(id)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeleteTermByUid, prefix="/load", endpoint="/terms/delete-uid")]
pub async fn delete_term_by_uid(uid: String) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .term_service
        .delete_by_uid(uid)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

// Info
#[server(name=LoadTermInfos, prefix="/load", endpoint="/terms/info")]
pub async fn load_term_infos(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<TermInfoTO>, ServerFnError> {
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
        .term_info_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(TermInfoTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountTermInfos, prefix="/load", endpoint="/terms/info/count")]
pub async fn count_term_infos(
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
        .term_info_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=GetTermInfoById, prefix="/load", endpoint="/terms/id/info")]
pub async fn get_term_info_by_id(id: i32) -> Result<TermInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .term_info_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(TermInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=GetTermInfoByUid, prefix="/load", endpoint="/terms/uid/info")]
pub async fn get_term_info_by_uid(uid: String) -> Result<TermInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .term_info_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(TermInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}
