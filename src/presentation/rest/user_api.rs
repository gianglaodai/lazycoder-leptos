#![cfg(feature = "ssr")]

use crate::business::user_service::User;
use crate::define_to_with_common_fields;
use crate::presentation::query_options::QueryOptions;
use crate::presentation::rest::response_result::{respond_result, respond_results};
use crate::state::AppState;
use actix_web::web::{scope, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, Responder};
use uuid::Uuid;

define_to_with_common_fields!(UserTO {
    pub username: String,
    pub email: String,
    #[serde(skip_serializing, default)]
    pub password: String
});

impl From<UserTO> for User {
    fn from(user: UserTO) -> Self {
        Self {
            id: user.id,
            uid: user.uid,
            created_at: user.created_at,
            updated_at: user.updated_at,
            username: user.username,
            email: user.email,
            password: user.password,
        }
    }
}

impl From<User> for UserTO {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            uid: user.uid,
            created_at: user.created_at,
            updated_at: user.updated_at,
            username: user.username,
            email: user.email,
            password: user.password,
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

#[post("")]
pub async fn create(state: Data<AppState>, user: Json<UserTO>) -> impl Responder {
    respond_result(
        state
            .user_service
            .create(&User::from(user.into_inner()))
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
pub async fn delete_by_uid(state: Data<AppState>, uid: Path<Uuid>) -> impl Responder {
    respond_result(state.user_service.delete_by_uid(uid.into_inner()).await)
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/users")
            .service(get_many)
            .service(get_by_id)
            .service(create)
            .service(update)
            .service(delete_by_id)
            .service(delete_by_uid),
    );
}
