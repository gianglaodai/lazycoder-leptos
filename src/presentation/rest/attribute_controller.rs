use crate::business::attribute_service::{Attribute, AttributeValueInfo};
use crate::define_readonly_to_with_common_fields_be;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Path, Query, ServiceConfig};
use actix_web::{get, Responder};

// attributes

define_readonly_to_with_common_fields_be!(Attribute {
    pub name: String,
    pub entity_type: String,
    pub data_type: String,
});

impl From<Attribute> for AttributeTO {
    fn from(entity: Attribute) -> Self { Self { id: entity.id, uid: entity.uid, version: entity.version, created_at: entity.created_at, updated_at: entity.updated_at, name: entity.name, entity_type: entity.entity_type, data_type: entity.data_type } }
}

#[get("")]
pub async fn get_many_attributes(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(state.attribute_service.get_many(query.to_sort_criteria(), query.first_result, query.max_results, query.to_filters()).await, AttributeTO::from)
}

#[get("/count")]
pub async fn count_attributes(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder { respond_result(state.attribute_service.count(query.to_filters()).await) }

#[get("/{id}")]
pub async fn get_attribute_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder { respond_result(state.attribute_service.get_by_id(id.into_inner()).await.map(|it| it.unwrap()).map(AttributeTO::from)) }

#[get("/uid/{uid}")]
pub async fn get_attribute_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder { respond_result(state.attribute_service.get_by_uid(uid.into_inner()).await.map(|it| it.unwrap()).map(AttributeTO::from)) }

pub fn routes_attributes(cfg: &mut ServiceConfig) { cfg.service(scope("/api/attributes").service(get_many_attributes).service(count_attributes).service(get_attribute_by_id).service(get_attribute_by_uid)); }

// attribute values

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

impl From<AttributeValueInfo> for AttributeValueInfoTO {
    fn from(entity: AttributeValueInfo) -> Self { Self { id: entity.id, uid: entity.uid, version: entity.version, created_at: entity.created_at, updated_at: entity.updated_at, int_value: entity.int_value, double_value: entity.double_value, string_value: entity.string_value, boolean_value: entity.boolean_value, date_value: entity.date_value, datetime_value: entity.datetime_value, time_value: entity.time_value, attribute_id: entity.attribute_id, attribute_name: entity.attribute_name, attribute_entity_type: entity.attribute_entity_type, attribute_data_type: entity.attribute_data_type, entity_id: entity.entity_id, entity_type: entity.entity_type } }
}

#[get("")]
pub async fn get_many_attribute_values(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder { respond_results(state.attribute_value_info_service.get_many(query.to_sort_criteria(), query.first_result, query.max_results, query.to_filters()).await, AttributeValueInfoTO::from) }

#[get("/count")]
pub async fn count_attribute_values(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder { respond_result(state.attribute_value_info_service.count(query.to_filters()).await) }

#[get("/{id}")]
pub async fn get_attribute_value_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder { respond_result(state.attribute_value_info_service.get_by_id(id.into_inner()).await.map(|it| it.unwrap()).map(AttributeValueInfoTO::from)) }

#[get("/uid/{uid}")]
pub async fn get_attribute_value_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder { respond_result(state.attribute_value_info_service.get_by_uid(uid.into_inner()).await.map(|it| it.unwrap()).map(AttributeValueInfoTO::from)) }

pub fn routes_attribute_values(cfg: &mut ServiceConfig) { cfg.service(scope("/api/attribute-values").service(get_many_attribute_values).service(count_attribute_values).service(get_attribute_value_by_id).service(get_attribute_value_by_uid)); }
