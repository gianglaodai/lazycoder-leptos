#![cfg(feature = "ssr")]
use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::post_term_service::{PostTermInfo, PostTermInfoRepository};
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxViewRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostTermInfoSqlxRepository {
    pool: PgPool,
}

define_readonly_orm_with_common_fields!(PostTermInfo {
    pub post_id: i32,
    pub term_id: i32,
    pub post_slug: Option<String>,
    pub post_title: Option<String>,
    pub term_slug: Option<String>,
    pub term_name: Option<String>,
    pub taxonomy_id: Option<i32>,
    pub taxonomy_code: Option<String>,
});

impl PostTermInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec![
            "post_slug",
            "post_title",
            "term_slug",
            "term_name",
            "taxonomy_code",
        ]
    }
}

impl From<PostTermInfoOrm> for PostTermInfo {
    fn from(orm: PostTermInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            post_id: orm.post_id,
            term_id: orm.term_id,
            post_slug: orm.post_slug,
            post_title: orm.post_title,
            term_slug: orm.term_slug,
            term_name: orm.term_name,
            taxonomy_id: orm.taxonomy_id,
            taxonomy_code: orm.taxonomy_code,
        }
    }
}

impl PostTermInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<PostTermInfo> for PostTermInfoSqlxRepository {
    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        SqlxViewRepository::count(self, filters).await
    }
    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostTermInfo>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }
    async fn find_by_id(&self, id: i32) -> Result<Option<PostTermInfo>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }
    async fn find_by_uid(&self, uid: String) -> Result<Option<PostTermInfo>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
}

impl SqlxViewRepository for PostTermInfoSqlxRepository {
    type Entity = PostTermInfo;
    type Orm = PostTermInfoOrm;
    fn get_table_name(&self) -> &str {
        "post_terms_info"
    }
    fn get_columns(&self) -> Vec<&'static str> {
        PostTermInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostTermInfoOrm::searchable_columns()
    }
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostTermInfo::from(orm)
    }
}

impl PostTermInfoRepository for PostTermInfoSqlxRepository {}
