use crate::business::attribute_value_service::{
    AttributeValue, AttributeValueCreate, AttributeValueInfo,
};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_fe, define_to_with_common_fields_fe};
use leptos::prelude::ServerFnError;
use leptos::*;

// Table TO
define_to_with_common_fields_fe!(AttributeValue {
    pub attribute_id: i32,
    pub entity_id: i32,
    pub entity_type: String,
    pub int_value: Option<i32>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub date_value: Option<time::Date>,
    pub datetime_value: Option<time::OffsetDateTime>,
    pub time_value: Option<time::Time>,
});
// View TO
define_readonly_to_with_common_fields_fe!(AttributeValueInfo {
    pub int_value: Option<i32>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub date_value: Option<time::Date>,
    pub datetime_value: Option<time::OffsetDateTime>,
    pub time_value: Option<time::Time>,
    pub attribute_id: i32,
    pub attribute_name: String,
    pub attribute_entity_type: String,
    pub attribute_data_type: String,
    pub entity_id: i32,
    pub entity_type: String,
});

impl From<AttributeValue> for AttributeValueTO {
    fn from(e: AttributeValue) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            attribute_id: e.attribute_id,
            entity_id: e.entity_id,
            entity_type: e.entity_type,
            int_value: e.int_value,
            double_value: e.double_value,
            string_value: e.string_value,
            boolean_value: e.boolean_value,
            date_value: e.date_value,
            datetime_value: e.datetime_value,
            time_value: e.time_value,
        }
    }
}
impl From<AttributeValueTO> for AttributeValue {
    fn from(t: AttributeValueTO) -> Self {
        Self {
            id: t.id,
            uid: t.uid,
            version: t.version,
            created_at: t.created_at,
            updated_at: t.updated_at,
            attribute_id: t.attribute_id,
            entity_id: t.entity_id,
            entity_type: t.entity_type,
            int_value: t.int_value,
            double_value: t.double_value,
            string_value: t.string_value,
            boolean_value: t.boolean_value,
            date_value: t.date_value,
            datetime_value: t.datetime_value,
            time_value: t.time_value,
        }
    }
}
impl From<AttributeValueInfo> for AttributeValueInfoTO {
    fn from(e: AttributeValueInfo) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            int_value: e.int_value,
            double_value: e.double_value,
            string_value: e.string_value,
            boolean_value: e.boolean_value,
            date_value: e.date_value,
            datetime_value: e.datetime_value,
            time_value: e.time_value,
            attribute_id: e.attribute_id,
            attribute_name: e.attribute_name,
            attribute_entity_type: e.attribute_entity_type,
            attribute_data_type: e.attribute_data_type,
            entity_id: e.entity_id,
            entity_type: e.entity_type,
        }
    }
}

#[server(name=LoadAttributeValues, prefix="/load", endpoint="/attribute_values")]
pub async fn load_attribute_values(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
) -> Result<Vec<AttributeValueTO>, ServerFnError> {
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
        a_filters,
    };
    state
        .attribute_value_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(AttributeValueTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountAttributeValues, prefix="/load", endpoint="/attribute_values/count")]
pub async fn count_attribute_values(
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
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
        a_filters,
    };
    state
        .attribute_value_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadAttributeValueById, prefix="/load", endpoint="/attribute_values/get")]
pub async fn load_attribute_value_by_id(id: i32) -> Result<AttributeValueTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_value_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(AttributeValueTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadAttributeValueByUid, prefix="/load", endpoint="/attribute_values/get-uid")]
pub async fn load_attribute_value_by_uid(uid: String) -> Result<AttributeValueTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_value_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(AttributeValueTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CreateAttributeValue, prefix="/load", endpoint="/attribute_values/create")]
pub async fn create_attribute_value(
    attribute_id: i32,
    entity_id: i32,
    entity_type: String,
) -> Result<AttributeValueTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let c = AttributeValueCreate {
        attribute_id,
        entity_id,
        entity_type,
    };
    state
        .attribute_value_service
        .create(&c)
        .await
        .map(AttributeValueTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=UpdateAttributeValue, prefix="/load", endpoint="/attribute_values/update")]
pub async fn update_attribute_value(
    entity: AttributeValueTO,
) -> Result<AttributeValueTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let e: AttributeValue = entity.into();
    state
        .attribute_value_service
        .update(&e)
        .await
        .map(AttributeValueTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeleteAttributeValueById, prefix="/load", endpoint="/attribute_values/delete")]
pub async fn delete_attribute_value_by_id(id: i32) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_value_service
        .delete_by_id(id)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeleteAttributeValueByUid, prefix="/load", endpoint="/attribute_values/delete-uid")]
pub async fn delete_attribute_value_by_uid(uid: String) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_value_service
        .delete_by_uid(uid)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

// Info
#[server(name=LoadAttributeValueInfos, prefix="/load", endpoint="/attribute_values/info")]
pub async fn load_attribute_value_infos(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
) -> Result<Vec<AttributeValueInfoTO>, ServerFnError> {
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
        a_filters,
    };
    state
        .attribute_value_info_service
        .get_many(
            q.to_sort_criteria(),
            q.first_result,
            q.max_results,
            q.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(AttributeValueInfoTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountAttributeValueInfos, prefix="/load", endpoint="/attribute_values/info/count")]
pub async fn count_attribute_value_infos(
    search: Option<String>,
    p_filters: Option<Vec<String>>,
    a_filters: Option<Vec<String>>,
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
        a_filters,
    };
    state
        .attribute_value_info_service
        .count(q.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadAttributeValueInfoById, prefix="/load", endpoint="/attribute_values/id/info")]
pub async fn load_attribute_value_info_by_id(
    id: i32,
) -> Result<AttributeValueInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_value_info_service
        .get_by_id(id)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(AttributeValueInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadAttributeValueInfoByUid, prefix="/load", endpoint="/attribute_values/uid/info")]
pub async fn load_attribute_value_info_by_uid(
    uid: String,
) -> Result<AttributeValueInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .attribute_value_info_service
        .get_by_uid(uid)
        .await
        .and_then(|o| o.ok_or(CoreError::not_found("error.not_found")))
        .map(AttributeValueInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}
