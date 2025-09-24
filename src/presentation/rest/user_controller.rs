#![cfg(feature = "ssr")]

use crate::business::user_service::{User, UserCreate, UserInfo, UserRole};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use crate::{define_readonly_to_with_common_fields_be, define_to_with_common_fields_be};
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};

define_to_with_common_fields_be!(User {
    pub username: String,
    pub email: String,
    #[serde(skip_serializing, default)]
    pub password: String,
    #[serde(skip_serializing, default)]
    pub role: i32,
});

// Readonly TO for info (no password)
define_readonly_to_with_common_fields_be!(UserInfo {
    pub username: String,
    pub email: String,
    pub role: String,
});

impl From<UserTO> for User {
    fn from(to: UserTO) -> Self {
        Self {
            id: to.id,
            uid: to.uid,
            version: to.version,
            created_at: to.created_at,
            updated_at: to.updated_at,
            username: to.username,
            email: to.email,
            password: to.password,
            role: UserRole::from(to.role),
        }
    }
}

impl From<UserCreateTO> for UserCreate {
    fn from(to: UserCreateTO) -> Self {
        Self {
            username: to.username,
            email: to.email,
            password: to.password,
            role: UserRole::USER,
        }
    }
}

impl From<User> for UserTO {
    fn from(entity: User) -> Self {
        Self {
            id: entity.id,
            uid: entity.uid,
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            username: entity.username,
            email: entity.email,
            password: entity.password,
            role: entity.role.as_i32(),
        }
    }
}

impl From<UserInfo> for UserInfoTO {
    fn from(e: UserInfo) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            username: e.username,
            email: e.email,
            role: e.role,
        }
    }
}

#[get("")]
pub async fn get_many(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .user_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        UserTO::from,
    )
}

#[get("/count")]
pub async fn count(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.user_service.count(query.to_filters()).await)
}

#[get("/{id}")]
pub async fn get_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .user_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(UserTO::from),
    )
}

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .user_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(UserTO::from),
    )
}

#[post("")]
pub async fn create(state: Data<AppState>, user: Json<UserCreateTO>) -> impl Responder {
    respond_result(
        state
            .user_service
            .create(&UserCreate::from(user.into_inner()))
            .await
            .map(UserTO::from),
    )
}

#[put("/{id}")]
pub async fn update(
    state: Data<AppState>,
    id: Path<i32>,
    mut user: Json<UserTO>,
) -> impl Responder {
    let mut body = user.into_inner();
    body.id = id.into_inner();
    respond_result(
        state
            .user_service
            .update(&User::from(body))
            .await
            .map(UserTO::from),
    )
}

#[delete("/{id}")]
pub async fn delete_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(state.user_service.delete_by_id(id.into_inner()).await)
}

#[delete("/uid/{uid}")]
pub async fn delete_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(state.user_service.delete_by_uid(uid.into_inner()).await)
}

// ===== Info endpoints =====
#[get("/info")]
pub async fn get_many_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_results(
        state
            .user_info_service
            .get_many(
                query.to_sort_criteria(),
                query.first_result,
                query.max_results,
                query.to_filters(),
            )
            .await,
        UserInfoTO::from,
    )
}

#[get("/info/count")]
pub async fn count_info(state: Data<AppState>, query: Query<QueryOptions>) -> impl Responder {
    respond_result(state.user_info_service.count(query.to_filters()).await)
}

#[get("/{id}/info")]
pub async fn get_info_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    respond_result(
        state
            .user_info_service
            .get_by_id(id.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(UserInfoTO::from),
    )
}

#[get("/uid/{uid}/info")]
pub async fn get_info_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .user_info_service
            .get_by_uid(uid.into_inner())
            .await
            .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
            .map(UserInfoTO::from),
    )
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
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
