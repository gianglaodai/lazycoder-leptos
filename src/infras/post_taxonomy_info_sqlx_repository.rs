#![cfg(feature = "ssr")]
use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::business::taxonomy_service::{PostTaxonomyInfo, PostTaxonomyInfoRepository};
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxViewRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostTaxonomyInfoSqlxRepository { pool: PgPool }

define_readonly_orm_with_common_fields!(PostTaxonomyInfo { pub code: String, pub name: String, });

impl PostTaxonomyInfoOrm { pub fn searchable_columns() -> Vec<&'static str> { vec!["code", "name"] } }

impl From<PostTaxonomyInfoOrm> for PostTaxonomyInfo {
    fn from(orm: PostTaxonomyInfoOrm) -> Self {
        Self { id: orm.id, uid: orm.uid.to_string(), version: orm.version, created_at: orm.created_at, updated_at: orm.updated_at, code: orm.code, name: orm.name }
    }
}

impl PostTaxonomyInfoSqlxRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

impl ViewRepository<PostTaxonomyInfo> for PostTaxonomyInfoSqlxRepository {
    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> { SqlxViewRepository::count(self, filters).await }
    async fn find_many(&self, sort_criteria: Vec<SortCriterion>, first_result: Option<i32>, max_results: Option<i32>, filters: Vec<Filter>) -> Result<Vec<PostTaxonomyInfo>, CoreError> { SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await }
    async fn find_by_id(&self, id: i32) -> Result<Option<PostTaxonomyInfo>, CoreError> { SqlxViewRepository::find_by_id(self, id).await }
    async fn find_by_uid(&self, uid: String) -> Result<Option<PostTaxonomyInfo>, CoreError> { SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await }
}

impl SqlxViewRepository for PostTaxonomyInfoSqlxRepository {
    type Entity = PostTaxonomyInfo; type Orm = PostTaxonomyInfoOrm;
    fn get_table_name(&self) -> &str { "post_taxonomies_info" }
    fn get_columns(&self) -> Vec<&'static str> { PostTaxonomyInfoOrm::columns() }
    fn get_searchable_columns(&self) -> Vec<&str> { PostTaxonomyInfoOrm::searchable_columns() }
    fn get_pool(&self) -> &PgPool { &self.pool }
    fn from_orm(orm: Self::Orm) -> Self::Entity { PostTaxonomyInfo::from(orm) }
}

impl PostTaxonomyInfoRepository for PostTaxonomyInfoSqlxRepository {}
