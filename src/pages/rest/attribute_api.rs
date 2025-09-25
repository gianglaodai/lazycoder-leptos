use crate::business::attribute_service::{Attribute, AttributeCreate, AttributeInfo};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_fe, define_to_with_common_fields_fe};
use leptos::prelude::ServerFnError;
use leptos::*;

// Table TO
define_to_with_common_fields_fe!(Attribute { pub name: String, pub entity_type: String, pub data_type: String, });
// View TO
define_readonly_to_with_common_fields_fe!(AttributeInfo { pub name: String, pub entity_type: String, pub data_type: String, });

impl From<Attribute> for AttributeTO {
    fn from(e: Attribute) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            name: e.name,
            entity_type: e.entity_type,
            data_type: e.data_type,
        }
    }
}
impl From<AttributeTO> for Attribute {
    fn from(t: AttributeTO) -> Self {
        Self {
            id: t.id,
            uid: t.uid,
            version: t.version,
            created_at: t.created_at,
            updated_at: t.updated_at,
            name: t.name,
            entity_type: t.entity_type,
            data_type: t.data_type,
        }
    }
}
impl From<AttributeInfo> for AttributeInfoTO {
    fn from(e: AttributeInfo) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            name: e.name,
            entity_type: e.entity_type,
            data_type: e.data_type,
        }
    }
}

#[server(name=LoadAttributes, prefix="/load", endpoint="/attributes")]
pub async fn load_attributes(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<AttributeTO>, ServerFnError> {
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
        .attribute_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(AttributeTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountAttributes, prefix="/load", endpoint="/attributes/count")]
pub async fn count_attributes(
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
        .attribute_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadAttributeById, prefix="/load", endpoint="/attributes/get")]
pub async fn load_attribute_by_id(id: i32) -> Result<AttributeTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(AttributeTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadAttributeByUid, prefix="/load", endpoint="/attributes/get-uid")]
pub async fn load_attribute_by_uid(uid: String) -> Result<AttributeTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(AttributeTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CreateAttribute, prefix="/load", endpoint="/attributes/create")]
pub async fn create_attribute(
    name: String,
    entity_type: String,
    data_type: String,
) -> Result<AttributeTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let c = AttributeCreate {
        name,
        entity_type,
        data_type,
    };
    state
        .attribute_service
        .create(&c)
        .await
        .map(AttributeTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=UpdateAttribute, prefix="/load", endpoint="/attributes/update")]
pub async fn update_attribute(entity: AttributeTO) -> Result<AttributeTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let e: Attribute = entity.into();
    state
        .attribute_service
        .update(&e)
        .await
        .map(AttributeTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeleteAttributeById, prefix="/load", endpoint="/attributes/delete")]
pub async fn delete_attribute_by_id(id: i32) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_service
        .delete_by_id(id)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeleteAttributeByUid, prefix="/load", endpoint="/attributes/delete-uid")]
pub async fn delete_attribute_by_uid(uid: String) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_service
        .delete_by_uid(uid)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

// Info
#[server(name=LoadAttributeInfos, prefix="/load", endpoint="/attributes/info")]
pub async fn load_attribute_infos(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<AttributeInfoTO>, ServerFnError> {
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
        .attribute_info_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(AttributeInfoTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountAttributeInfos, prefix="/load", endpoint="/attributes/info/count")]
pub async fn count_attribute_infos(
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
        .attribute_info_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadAttributeInfoById, prefix="/load", endpoint="/attributes/id/info")]
pub async fn load_attribute_info_by_id(id: i32) -> Result<AttributeInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_info_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(AttributeInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadAttributeInfoByUid, prefix="/load", endpoint="/attributes/uid/info")]
pub async fn load_attribute_info_by_uid(uid: String) -> Result<AttributeInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_info_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(AttributeInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}
