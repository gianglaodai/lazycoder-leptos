#![cfg(feature = "ssr")]

use crate::business::user_service::{User, UserCreate};
use crate::define_to_with_common_fields_be;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};

define_to_with_common_fields_be!(User {
    pub username: String,
    pub email: String,
    #[serde(skip_serializing, default)]
    pub password: String
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
        }
    }
}

impl From<UserCreateTO> for UserCreate {
    fn from(to: UserCreateTO) -> Self {
        Self {
            username: to.username,
            email: to.email,
            password: to.password,
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
            .map(|user| user.map(UserTO::from)),
    )
}

#[get("/uid/{uid}")]
pub async fn get_by_uid(state: Data<AppState>, uid: Path<String>) -> impl Responder {
    respond_result(
        state
            .user_service
            .get_by_uid(uid.into_inner())
            .await
            .map(|user| user.map(UserTO::from)),
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
pub async fn update(state: Data<AppState>, user: Json<UserTO>) -> impl Responder {
    respond_result(
        state
            .user_service
            .update(&User::from(user.into_inner()))
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

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .service(get_many)
            .service(get_by_id)
            .service(get_by_uid)
            .service(create)
            .service(update)
            .service(delete_by_id)
            .service(delete_by_uid),
    );
}
