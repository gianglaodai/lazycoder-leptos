use crate::business::user_service::{User, UserCreate, UserInfo, UserRole};
use crate::common::error::CoreError;
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_to_with_common_fields_fe, define_to_with_common_fields_fe};
use leptos::prelude::ServerFnError;
use leptos::*;

// Table TO (no password in info endpoints; for create we accept password)
define_to_with_common_fields_fe!(User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
});

// Readonly TO for info (no password)
define_readonly_to_with_common_fields_fe!(UserInfo {
    pub username: String,
    pub email: String,
    pub role: String,
});

impl From<User> for UserTO {
    fn from(e: User) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            version: e.version,
            created_at: e.created_at,
            updated_at: e.updated_at,
            username: e.username,
            email: e.email,
            password: e.password,
            role: e.role.as_str().to_string(),
        }
    }
}
impl From<UserTO> for User {
    fn from(to: UserTO) -> Self {
        let role = match to.role.as_str() {
            "ADMIN" => UserRole::ADMIN,
            _ => UserRole::USER,
        };
        Self {
            id: to.id,
            uid: to.uid,
            version: to.version,
            created_at: to.created_at,
            updated_at: to.updated_at,
            username: to.username,
            email: to.email,
            password: to.password,
            role,
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

#[server(name=LoadUsers, prefix="/load", endpoint="/users")]
pub async fn load_users(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<UserTO>, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;

    let state: Data<AppState> = extract().await?;
    let query = QueryOptions {
        first_result,
        max_results,
        sort,
        search,
        p_filters,
        a_filters: None,
    };
    state
        .user_service
        .get_many(
            query.to_sort_criteria(),
            query.first_result,
            query.max_results,
            query.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(UserTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountUsers, prefix="/load", endpoint="/users/count")]
pub async fn count_users(
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<i64, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let query = QueryOptions {
        first_result: None,
        max_results: None,
        sort: None,
        search,
        p_filters,
        a_filters: None,
    };
    state
        .user_service
        .count(query.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadUserById, prefix="/load", endpoint="/users/get")]
pub async fn load_user_by_id(id: i32) -> Result<UserTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .user_service
        .get_by_id(id)
        .await
        .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
        .map(UserTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadUserByUid, prefix="/load", endpoint="/users/get-uid")]
pub async fn load_user_by_uid(uid: String) -> Result<UserTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .user_service
        .get_by_uid(uid)
        .await
        .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
        .map(UserTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CreateUser, prefix="/load", endpoint="/users/create")]
pub async fn create_user(
    username: String,
    email: String,
    password: String,
) -> Result<UserTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let create = UserCreate {
        username,
        email,
        password,
        role: UserRole::USER,
    };
    state
        .user_service
        .create(&create)
        .await
        .map(UserTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=UpdateUser, prefix="/load", endpoint="/users/update")]
pub async fn update_user(user: UserTO) -> Result<UserTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let entity: User = user.into();
    state
        .user_service
        .update(&entity)
        .await
        .map(UserTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeleteUserById, prefix="/load", endpoint="/users/delete")]
pub async fn delete_user_by_id(id: i32) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .user_service
        .delete_by_id(id)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=DeleteUserByUid, prefix="/load", endpoint="/users/delete-uid")]
pub async fn delete_user_by_uid(uid: String) -> Result<u64, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .user_service
        .delete_by_uid(uid)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

// Info endpoints
#[server(name=LoadUserInfos, prefix="/load", endpoint="/users/info")]
pub async fn load_user_infos(
    first_result: Option<i32>,
    max_results: Option<i32>,
    sort: Option<String>,
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<Vec<UserInfoTO>, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let query = QueryOptions {
        first_result,
        max_results,
        sort,
        search,
        p_filters,
        a_filters: None,
    };
    state
        .user_info_service
        .get_many(
            query.to_sort_criteria(),
            query.first_result,
            query.max_results,
            query.to_filters(),
        )
        .await
        .map(|v| v.into_iter().map(UserInfoTO::from).collect())
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=CountUserInfos, prefix="/load", endpoint="/users/info/count")]
pub async fn count_user_infos(
    search: Option<String>,
    p_filters: Option<Vec<String>>,
) -> Result<i64, ServerFnError> {
    use crate::presentation::query_options::QueryOptions;
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let query = QueryOptions {
        first_result: None,
        max_results: None,
        sort: None,
        search,
        p_filters,
        a_filters: None,
    };
    state
        .user_info_service
        .count(query.to_filters())
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadUserInfoById, prefix="/load", endpoint="/users/id/info")]
pub async fn load_user_info_by_id(id: i32) -> Result<UserInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .user_info_service
        .get_by_id(id)
        .await
        .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
        .map(UserInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}

#[server(name=LoadUserInfoByUid, prefix="/load", endpoint="/users/uid/info")]
pub async fn load_user_info_by_uid(uid: String) -> Result<UserInfoTO, ServerFnError> {
    use crate::state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    state
        .user_info_service
        .get_by_uid(uid)
        .await
        .and_then(|opt| opt.ok_or(CoreError::not_found("error.not_found")))
        .map(UserInfoTO::from)
        .map_err(|e| ServerFnError::ServerError(e.to_json()))
}
