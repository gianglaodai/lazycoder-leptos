#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::post_service::{PostInfo, PostStatus};
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxViewRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostInfoSqlxRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(PostInfo {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: i32,
    pub user_id: i32,
    pub username: String,
    pub email: String,
});

impl From<PostInfoOrm> for PostInfo {
    fn from(orm: PostInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            slug: orm.slug,
            title: orm.title,
            summary: orm.summary,
            content: orm.content,
            status: PostStatus::from(orm.status),
            user_id: orm.user_id,
            username: orm.username,
            email: orm.email,
        }
    }
}

impl PostInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["slug", "title", "summary", "content"]
    }
}

impl PostInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<PostInfo> for PostInfoSqlxRepository {
    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        SqlxViewRepository::count(self, filters).await
    }

    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostInfo>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PostInfo>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }

    async fn find_by_uid(&self, uid: String) -> Result<Option<PostInfo>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
}

impl SqlxViewRepository for PostInfoSqlxRepository {
    type Entity = PostInfo;
    type Orm = PostInfoOrm;
    fn get_table_name(&self) -> &str {
        "posts_info"
    }
    fn get_columns(&self) -> Vec<&'static str> {
        PostInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostInfoOrm::searchable_columns()
    }
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostInfo::from(orm)
    }
}
