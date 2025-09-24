use crate::business::attribute_service::{Attribute, AttributeCreate, AttributeInfo};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use crate::{define_readonly_to_with_common_fields_be, define_to_with_common_fields_be};
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};
// attributes

// Table TO (CRUD)
define_to_with_common_fields_be!(Attribute {
    pub name: String,
    pub entity_type: String,
    pub data_type: String,
});

// View TO (info)
define_readonly_to_with_common_fields_be!(AttributeInfo {
    pub name: String,
    pub entity_type: String,
    pub data_type: String,
});

impl From<AttributeTO> for Attribute {
    fn from(to: AttributeTO) -> Self {
        Self {
            id: to.id,
            uid: to.uid,
            version: to.version,
            created_at: to.created_at,
            updated_at: to.updated_at,
            name: to.name,
            entity_type: to.entity_type,
            data_type: to.data_type,
        }
    }
}
impl From<Attribute> for AttributeTO {
    fn from(entity: Attribute) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            name: entity.name,
            entity_type: entity.entity_type,
            data_type: entity.data_type,
        }
    }
}
impl From<AttributeInfo> for AttributeInfoTO {
    fn from(entity: AttributeInfo) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            name: entity.name,
            entity_type: entity.entity_type,
            data_type: entity.data_type,
        }
    }
}
impl From<AttributeCreateTO> for AttributeCreate {
    fn from(to: AttributeCreateTO) -> Self {
        Self {
            name: to.name,
            entity_type: to.entity_type,
            data_type: to.data_type,
        }
    }
}

#[get("")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .attribute_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        AttributeTO::from,
    )
}

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.attribute_service.count(query.to_filters()).await)
}

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .attribute_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(AttributeTO::from),
    )
}

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .attribute_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(AttributeTO::from),
    )
}

#[post("")]
pub async fn create(state: Data<AppState>, data: Json<AttributeCreateTO>) -> impl Responder {
    respond_result(
        state
            .attribute_service
            .create(&AttributeCreate::from(data.into_inner()))
            .await
            .map(AttributeTO::from),
    )
}

#[put("/{id}")]
pub async fn update(
    state: Data<AppState>,
    id: Path<i32>,
    mut data: Json<AttributeTO>,
) -> impl Responder {
    let mut body = data.into_inner();
    body.id = id.into_inner();
    respond_result(
        state
            .attribute_service
            .update(&Attribute::from(body))
            .await
            .map(AttributeTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(state.attribute_service.delete_by_id(id.into_inner()).await)
}

#[delete("/uid/{uid}")]
pub async fn delete_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .attribute_service
            .delete_by_uid(uid.into_inner())
            .await,
    )
}

// Info endpoints
#[get("/info")]
pub async fn get_many_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .attribute_info_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        AttributeInfoTO::from,
    )
}

#[get("/info/count")]
pub async fn count_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.attribute_info_service.count(query.to_filters()).await)
}

#[get("/{id}/info")]
pub async fn get_info_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .attribute_info_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(AttributeInfoTO::from),
    )
}

#[get("/uid/{uid}/info")]
pub async fn get_info_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .attribute_info_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(AttributeInfoTO::from),
    )
}

pub fn routes_attributes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api/attributes")
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
