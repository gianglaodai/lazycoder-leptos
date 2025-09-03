#![cfg(feature = "ssr")]
use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::business::taxonomy_service::{TermInfo, TermInfoRepository};
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxViewRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct TermInfoSqlxRepository { pool: PgPool }

define_readonly_orm_with_common_fields!(TermInfo {
    pub taxonomy_id: i32,
    pub taxonomy_code: String,
    pub taxonomy_name: String,
    pub parent_id: Option<i32>,
    pub parent_slug: Option<String>,
    pub parent_name: Option<String>,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
});

impl TermInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> { vec!["slug", "name", "taxonomy_code", "taxonomy_name"] }
}

impl From<TermInfoOrm> for TermInfo {
    fn from(orm: TermInfoOrm) -> Self {
        Self { id: orm.id, uid: orm.uid.to_string(), version: orm.version, created_at: orm.created_at, updated_at: orm.updated_at, taxonomy_id: orm.taxonomy_id, taxonomy_code: orm.taxonomy_code, taxonomy_name: orm.taxonomy_name, parent_id: orm.parent_id, parent_slug: orm.parent_slug, parent_name: orm.parent_name, slug: orm.slug, name: orm.name, description: orm.description }
    }
}

impl TermInfoSqlxRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

impl ViewRepository<TermInfo> for TermInfoSqlxRepository {
    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> { SqlxViewRepository::count(self, filters).await }
    async fn find_many(&self, sort_criteria: Vec<SortCriterion>, first_result: Option<i32>, max_results: Option<i32>, filters: Vec<Filter>) -> Result<Vec<TermInfo>, CoreError> { SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await }
    async fn find_by_id(&self, id: i32) -> Result<Option<TermInfo>, CoreError> { SqlxViewRepository::find_by_id(self, id).await }
    async fn find_by_uid(&self, uid: String) -> Result<Option<TermInfo>, CoreError> { SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await }
}

impl SqlxViewRepository for TermInfoSqlxRepository {
    type Entity = TermInfo; type Orm = TermInfoOrm;
    fn get_table_name(&self) -> &str { "terms_info" }
    fn get_columns(&self) -> Vec<&'static str> { TermInfoOrm::columns() }
    fn get_searchable_columns(&self) -> Vec<&str> { TermInfoOrm::searchable_columns() }
    fn get_pool(&self) -> &PgPool { &self.pool }
    fn from_orm(orm: Self::Orm) -> Self::Entity { TermInfo::from(orm) }
}

impl TermInfoRepository for TermInfoSqlxRepository {}
