#![cfg(feature = "ssr")]
use crate::business::attribute_service::{AttributeInfoService, AttributeService};
use crate::business::attribute_value_service::{AttributeValueInfoService, AttributeValueService};
use crate::business::auth_service::AuthService;
use crate::business::post_collection_service::{PostCollectionInfoService, PostCollectionService};
use crate::business::post_service::{PostInfoService, PostService};
use crate::business::post_taxonomy_service::{PostTaxonomyInfoService, PostTaxonomyService};
use crate::business::post_type_service::{PostTypeInfoService, PostTypeService};
use crate::business::term_service::{TermInfoService, TermService};
use crate::business::user_service::{UserService, UserInfoService};
use crate::infras::attribute_sqlx_repository::{AttributeInfoSqlxRepository, AttributeSqlxRepository};
use crate::infras::attribute_value_sqlx_repository::{AttributeValueInfoSqlxRepository, AttributeValueSqlxRepository};
use crate::infras::post_collection_sqlx_repository::{PostCollectionInfoSqlxRepository, PostCollectionSqlxRepository};
use crate::infras::post_sqlx_repository::{PostInfoSqlxRepository, PostSqlxRepository};
use crate::infras::post_taxonomy_sqlx_repository::{PostTaxonomyInfoSqlxRepository, PostTaxonomySqlxRepository};
use crate::infras::post_type_sqlx_repository::{PostTypeInfoSqlxRepository, PostTypeSqlxRepository};
use crate::infras::term_sqlx_repository::{TermInfoSqlxRepository, TermSqlxRepository};
use crate::infras::user_sqlx_repository::{UserSqlxRepository, UserInfoSqlxRepository};
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    // Services for table CRUD
    pub auth_service: AuthService<UserSqlxRepository>,
    pub user_service: UserService<UserSqlxRepository>,
    pub post_service: PostService<PostSqlxRepository>,
    pub post_type_service: PostTypeService<PostTypeSqlxRepository>,
    pub post_collection_service: PostCollectionService<PostCollectionSqlxRepository>,
    pub post_taxonomy_service: PostTaxonomyService<PostTaxonomySqlxRepository>,
    pub term_service: TermService<TermSqlxRepository>,
    pub attribute_value_service: AttributeValueService<AttributeValueSqlxRepository>,
    pub attribute_service: AttributeService<AttributeSqlxRepository>,
    // View services
    pub user_info_service: UserInfoService<UserInfoSqlxRepository>,
    pub post_info_service: PostInfoService<PostInfoSqlxRepository>,
    pub post_type_info_service: PostTypeInfoService<PostTypeInfoSqlxRepository>,
    pub post_collection_info_service: PostCollectionInfoService<PostCollectionInfoSqlxRepository>,
    pub post_taxonomy_info_service: PostTaxonomyInfoService<PostTaxonomyInfoSqlxRepository>,
    pub term_info_service: TermInfoService<TermInfoSqlxRepository>,
    pub attribute_info_service: AttributeInfoService<AttributeInfoSqlxRepository>,
    pub attribute_value_info_service: AttributeValueInfoService<AttributeValueInfoSqlxRepository>,
}

#[cfg(feature = "ssr")]
pub async fn new_app_state(pool: PgPool) -> actix_web::web::Data<AppState> {
    let user_repository = Arc::new(UserSqlxRepository::new(pool.clone()));
    let user_service = UserService::new(user_repository);
    let user_info_repository = Arc::new(UserInfoSqlxRepository::new(pool.clone()));
    let user_info_service = UserInfoService::new(user_info_repository);
    let post_repository = Arc::new(PostSqlxRepository::new(pool.clone()));
    let post_service = PostService::new(post_repository);
    let post_info_repository = Arc::new(PostInfoSqlxRepository::new(pool.clone()));
    let post_info_service = PostInfoService::new(post_info_repository);
    let post_type_info_repository = Arc::new(PostTypeInfoSqlxRepository::new(pool.clone()));
    let post_type_info_service = PostTypeInfoService::new(post_type_info_repository);
    let post_collection_info_repository =
        Arc::new(PostCollectionInfoSqlxRepository::new(pool.clone()));
    let post_collection_info_service =
        PostCollectionInfoService::new(post_collection_info_repository);
    let post_taxonomy_info_repository = Arc::new(PostTaxonomyInfoSqlxRepository::new(pool.clone()));
    let post_taxonomy_info_service = PostTaxonomyInfoService::new(post_taxonomy_info_repository);
    let term_info_repository = Arc::new(TermInfoSqlxRepository::new(pool.clone()));
    let term_info_service = TermInfoService::new(term_info_repository);

    let attribute_repository = Arc::new(AttributeSqlxRepository::new(pool.clone()));
    let attribute_service = AttributeService::new(attribute_repository);
    let attribute_info_repository = Arc::new(AttributeInfoSqlxRepository::new(pool.clone()));
    let attribute_info_service = AttributeInfoService::new(attribute_info_repository);

    let attribute_value_info_repository =
        Arc::new(AttributeValueInfoSqlxRepository::new(pool.clone()));
    let attribute_value_info_service =
        AttributeValueInfoService::new(attribute_value_info_repository);
    let post_type_repository = Arc::new(PostTypeSqlxRepository::new(pool.clone()));
    let post_type_service = PostTypeService::new(post_type_repository);

    let post_collection_repository = Arc::new(PostCollectionSqlxRepository::new(pool.clone()));
    let post_collection_service = PostCollectionService::new(post_collection_repository);

    let post_taxonomy_repository = Arc::new(PostTaxonomySqlxRepository::new(pool.clone()));
    let post_taxonomy_service = PostTaxonomyService::new(post_taxonomy_repository);

    let term_repository = Arc::new(TermSqlxRepository::new(pool.clone()));
    let term_service = TermService::new(term_repository);

    let attribute_value_repository = Arc::new(AttributeValueSqlxRepository::new(pool.clone()));
    let attribute_value_service = AttributeValueService::new(attribute_value_repository);

    let auth_service = AuthService::new(user_service.clone());

    actix_web::web::Data::new(AppState {
        // CRUD services
        user_service,
        user_info_service,
        post_service,
        auth_service,
        post_type_service,
        post_collection_service,
        post_taxonomy_service,
        term_service,
        attribute_value_service,
        attribute_service,
        // View services
        post_info_service,
        post_type_info_service,
        post_collection_info_service,
        post_taxonomy_info_service,
        term_info_service,
        attribute_info_service,
        attribute_value_info_service,
    })
}
