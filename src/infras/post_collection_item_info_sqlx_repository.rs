#![cfg(feature = "ssr")]
use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::business::collection_item_service::{PostCollectionItemInfo, PostCollectionItemInfoRepository};
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxViewRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostCollectionItemInfoSqlxRepository { pool: PgPool }

define_readonly_orm_with_common_fields!(PostCollectionItemInfo {
    pub post_collection_id: i32,
    pub post_id: i32,
    pub position: i32,
    pub headline: Option<String>,
    pub collection_slug: Option<String>,
    pub collection_title: Option<String>,
    pub post_slug: Option<String>,
    pub post_title: Option<String>,
});

impl PostCollectionItemInfoOrm { pub fn searchable_columns() -> Vec<&'static str> { vec!["collection_slug", "collection_title", "post_slug", "post_title"] } }

impl From<PostCollectionItemInfoOrm> for PostCollectionItemInfo {
    fn from(orm: PostCollectionItemInfoOrm) -> Self { Self { id: orm.id, uid: orm.uid.to_string(), version: orm.version, created_at: orm.created_at, updated_at: orm.updated_at, post_collection_id: orm.post_collection_id, post_id: orm.post_id, position: orm.position, headline: orm.headline, collection_slug: orm.collection_slug, collection_title: orm.collection_title, post_slug: orm.post_slug, post_title: orm.post_title } }
}

impl PostCollectionItemInfoSqlxRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

impl ViewRepository<PostCollectionItemInfo> for PostCollectionItemInfoSqlxRepository {
    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> { SqlxViewRepository::count(self, filters).await }
    async fn find_many(&self, sort_criteria: Vec<SortCriterion>, first_result: Option<i32>, max_results: Option<i32>, filters: Vec<Filter>) -> Result<Vec<PostCollectionItemInfo>, CoreError> { SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await }
    async fn find_by_id(&self, id: i32) -> Result<Option<PostCollectionItemInfo>, CoreError> { SqlxViewRepository::find_by_id(self, id).await }
    async fn find_by_uid(&self, uid: String) -> Result<Option<PostCollectionItemInfo>, CoreError> { SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await }
}

impl SqlxViewRepository for PostCollectionItemInfoSqlxRepository {
    type Entity = PostCollectionItemInfo; type Orm = PostCollectionItemInfoOrm;
    fn get_table_name(&self) -> &str { "post_collection_items_info" }
    fn get_columns(&self) -> Vec<&'static str> { PostCollectionItemInfoOrm::columns() }
    fn get_searchable_columns(&self) -> Vec<&str> { PostCollectionItemInfoOrm::searchable_columns() }
    fn get_pool(&self) -> &PgPool { &self.pool }
    fn from_orm(orm: Self::Orm) -> Self::Entity { PostCollectionItemInfo::from(orm) }
}

impl PostCollectionItemInfoRepository for PostCollectionItemInfoSqlxRepository {}