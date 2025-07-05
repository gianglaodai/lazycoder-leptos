#![cfg(feature = "ssr")]
use crate::business::user_service::UserService;
use crate::infras::user_sqlx_repository::UserSqlxRepository;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub user_service: UserService<UserSqlxRepository>,
}

#[cfg(feature = "ssr")]
pub async fn new_app_state(pool: PgPool) -> actix_web::web::Data<AppState> {
    let user_repository = Arc::new(UserSqlxRepository::new(pool.clone()));
    let user_service = UserService::new(user_repository);

    actix_web::web::Data::new(AppState { user_service })
}
