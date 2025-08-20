#![cfg(feature = "ssr")]
use crate::business::auth_service::AuthService;
use crate::business::post_service::{PostInfoService, PostService};
use crate::business::user_service::UserService;
use crate::infras::post_info_sqlx_repository::PostInfoSqlxRepository;
use crate::infras::post_sqlx_repository::PostSqlxRepository;
use crate::infras::user_sqlx_repository::UserSqlxRepository;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub user_service: UserService<UserSqlxRepository>,
    pub post_service: PostService<PostSqlxRepository>,
    pub auth_service: AuthService<UserSqlxRepository>,
}

#[cfg(feature = "ssr")]
pub async fn new_app_state(pool: PgPool) -> actix_web::web::Data<AppState> {
    let user_repository = Arc::new(UserSqlxRepository::new(pool.clone()));
    let user_service = UserService::new(user_repository);
    let post_repository = Arc::new(PostSqlxRepository::new(pool.clone()));
    let post_service = PostService::new(post_repository);
    let auth_service = AuthService::new(user_service.clone());

    actix_web::web::Data::new(AppState {
        user_service,
        post_service,
        auth_service,
    })
}
