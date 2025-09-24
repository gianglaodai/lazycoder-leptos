#![cfg(feature = "ssr")]

use crate::business::attribute_value_service::{
    AttributeValue, AttributeValueCreate, AttributeValueInfo,
};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use crate::{define_readonly_to_with_common_fields_be, define_to_with_common_fields_be};
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};

// Table TO for CRUD
define_to_with_common_fields_be!(AttributeValue {
    req {
        pub attribute_id: i32,
        pub entity_id: i32,
        pub entity_type: String,
    }
    opt {
        pub int_value: Option<i32>,
        pub double_value: Option<f64>,
        pub string_value: Option<String>,
        pub boolean_value: Option<bool>,
        pub date_value: Option<time::Date>,
        pub datetime_value: Option<time::OffsetDateTime>,
        pub time_value: Option<time::Time>,
    }
});

// View TO for info
define_readonly_to_with_common_fields_be!(AttributeValueInfo {
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

impl From<AttributeValueTO> for AttributeValue {
    fn from(to: AttributeValueTO) -> Self {
        Self {
            id: to.id,
            uid: to.uid,
            version: to.version,
            created_at: to.created_at,
            updated_at: to.updated_at,
            attribute_id: to.attribute_id,
            entity_id: to.entity_id,
            entity_type: to.entity_type,
            int_value: to.int_value,
            double_value: to.double_value,
            string_value: to.string_value,
            boolean_value: to.boolean_value,
            date_value: to.date_value,
            datetime_value: to.datetime_value,
            time_value: to.time_value,
        }
    }
}
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

// ===== Table endpoints =====
#[get("")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .attribute_value_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        AttributeValueTO::from,
    )
}

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(
        state
            .attribute_value_service
            .count(query.to_filters())
            .await,
    )
}

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .attribute_value_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(AttributeValueTO::from),
    )
}

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .attribute_value_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(AttributeValueTO::from),
    )
}

#[post("")]
pub async fn create(state: Data<AppState>, data: Json<AttributeValueCreateTO>) -> impl Responder {
    respond_result(
        state
            .attribute_value_service
            .create(&AttributeValueCreate::from(data.into_inner()))
            .await
            .map(AttributeValueTO::from),
    )
}

#[put("/{id}")]
pub async fn update(state: Data<AppState>, data: Json<AttributeValueTO>) -> impl Responder {
    respond_result(
        state
            .attribute_value_service
            .update(&AttributeValue::from(data.into_inner()))
            .await
            .map(AttributeValueTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .attribute_value_service
            .delete_by_id(id.into_inner())
            .await,
    )
}

#[delete("/uid/{uid}")]
pub async fn delete_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .attribute_value_service
            .delete_by_uid(uid.into_inner())
            .await,
    )
}

// ===== Info endpoints =====
#[get("/info")]
pub async fn get_many_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .attribute_value_info_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        AttributeValueInfoTO::from,
    )
}

#[get("/info/count")]
pub async fn count_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(
        state
            .attribute_value_info_service
            .count(query.to_filters())
            .await,
    )
}

#[get("/{id}/info")]
pub async fn get_info_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .attribute_value_info_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(AttributeValueInfoTO::from),
    )
}

#[get("/uid/{uid}/info")]
pub async fn get_info_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .attribute_value_info_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(AttributeValueInfoTO::from),
    )
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/attribute_values")
            .service(get_many)
            .service(count)
            .service(get_by_id)
            .service(get_by_uid)
            .service(create)
            .service(update)
            .service(delete_by_id)
            .service(delete_by_uid)
            .service(get_many_info)
            .service(count_info)
            .service(get_info_by_id)
            .service(get_info_by_uid),
    );
}

impl From<AttributeValueCreateTO> for AttributeValueCreate {
    fn from(to: AttributeValueCreateTO) -> Self {
        Self {
            attribute_id: to.attribute_id,
            entity_id: to.entity_id,
            entity_type: to.entity_type,
        }
    }
}
