#![cfg(feature = "ssr")]
use crate::business::attribute_service::AttributeService;
use crate::business::attribute_value_service::AttributeValueInfoService;
use crate::business::auth_service::AuthService;
use crate::business::post_collection_item_service::PostCollectionItemInfoService;
use crate::business::post_collection_service::PostCollectionInfoService;
use crate::business::post_relation_service::PostRelationInfoService;
use crate::business::post_service::{PostInfoService, PostService};
use crate::business::post_taxonomy_service::PostTaxonomyInfoService;
use crate::business::post_term_service::PostTermInfoService;
use crate::business::post_type_service::{PostTypeInfoService, PostTypeService};
use crate::business::term_service::TermInfoService;
use crate::business::user_service::UserService;
use crate::infras::attribute_sqlx_repository::AttributeSqlxRepository;
use crate::infras::attribute_value_sqlx_repository::AttributeValueInfoSqlxRepository;
use crate::infras::post_collection_item_sqlx_repository::PostCollectionItemInfoSqlxRepository;
use crate::infras::post_collection_sqlx_repository::PostCollectionInfoSqlxRepository;
use crate::infras::post_relation_sqlx_repository::PostRelationInfoSqlxRepository;
use crate::infras::post_sqlx_repository::PostInfoSqlxRepository;
use crate::infras::post_sqlx_repository::PostSqlxRepository;
use crate::infras::post_taxonomy_sqlx_repository::PostTaxonomyInfoSqlxRepository;
use crate::infras::post_term_sqlx_repository::PostTermInfoSqlxRepository;
use crate::infras::post_type_sqlx_repository::PostTypeInfoSqlxRepository;
use crate::infras::post_type_sqlx_repository::PostTypeSqlxRepository;
use crate::infras::term_sqlx_repository::TermInfoSqlxRepository;
use crate::infras::user_sqlx_repository::UserSqlxRepository;
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub user_service: UserService<UserSqlxRepository>,
    pub auth_service: AuthService<UserSqlxRepository>,
    pub post_service: PostService<PostSqlxRepository>,
    pub post_info_service: PostInfoService<PostInfoSqlxRepository>,
    pub post_type_info_service: PostTypeInfoService<PostTypeInfoSqlxRepository>,
    pub post_collection_info_service: PostCollectionInfoService<PostCollectionInfoSqlxRepository>,
    pub post_taxonomy_info_service: PostTaxonomyInfoService<PostTaxonomyInfoSqlxRepository>,
    pub term_info_service: TermInfoService<TermInfoSqlxRepository>,
    pub attribute_service: AttributeService<AttributeSqlxRepository>,
    pub attribute_value_info_service: AttributeValueInfoService<AttributeValueInfoSqlxRepository>,
    pub post_relation_info_service: PostRelationInfoService<PostRelationInfoSqlxRepository>,
    pub post_term_info_service: PostTermInfoService<PostTermInfoSqlxRepository>,
    pub post_collection_item_info_service:
        PostCollectionItemInfoService<PostCollectionItemInfoSqlxRepository>,
    pub post_type_service: PostTypeService<PostTypeSqlxRepository>,
}

#[cfg(feature = "ssr")]
pub async fn new_app_state(pool: PgPool) -> actix_web::web::Data<AppState> {
    let user_repository = Arc::new(UserSqlxRepository::new(pool.clone()));
    let user_service = UserService::new(user_repository);
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
    let attribute_value_info_repository =
        Arc::new(AttributeValueInfoSqlxRepository::new(pool.clone()));
    let attribute_value_info_service =
        AttributeValueInfoService::new(attribute_value_info_repository);
    let post_relation_info_repository = Arc::new(PostRelationInfoSqlxRepository::new(pool.clone()));
    let post_relation_info_service = PostRelationInfoService::new(post_relation_info_repository);
    let post_term_info_repository = Arc::new(PostTermInfoSqlxRepository::new(pool.clone()));
    let post_term_info_service = PostTermInfoService::new(post_term_info_repository);
    let post_collection_item_info_repository =
        Arc::new(PostCollectionItemInfoSqlxRepository::new(pool.clone()));
    let post_collection_item_info_service =
        PostCollectionItemInfoService::new(post_collection_item_info_repository);
    let post_type_repository = Arc::new(PostTypeSqlxRepository::new(pool.clone()));
    let post_type_service = PostTypeService::new(post_type_repository);
    let auth_service = AuthService::new(user_service.clone());

    actix_web::web::Data::new(AppState {
        user_service,
        post_service,
        auth_service,
        post_info_service,
        post_type_info_service,
        post_collection_info_service,
        post_taxonomy_info_service,
        term_info_service,
        attribute_service,
        attribute_value_info_service,
        post_relation_info_service,
        post_term_info_service,
        post_collection_item_info_service,
        post_type_service,
    })
}
