#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::post_type_service::{PostTypeInfo, PostTypeInfoRepository};
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxViewRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostTypeInfoSqlxRepository {
    pool: PgPool,
}

define_readonly_orm_with_common_fields!(PostTypeInfo {
    pub code: String,
    pub name: String,
});

impl PostTypeInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["code", "name"]
    }
}

impl From<PostTypeInfoOrm> for PostTypeInfo {
    fn from(orm: PostTypeInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            code: orm.code,
            name: orm.name,
        }
    }
}

impl PostTypeInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<PostTypeInfo> for PostTypeInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_types_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostTypeInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostTypeInfoOrm::searchable_columns()
    }

    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        SqlxViewRepository::count(self, filters).await
    }

    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostTypeInfo>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PostTypeInfo>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }

    async fn find_by_uid(&self, uid: String) -> Result<Option<PostTypeInfo>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }

    async fn get_column_type_map(
        &self,
    ) -> Result<std::collections::HashMap<String, crate::business::filter::ScalarValue>, CoreError>
    {
        SqlxViewRepository::get_column_type_map(self).await
    }
}

impl SqlxViewRepository for PostTypeInfoSqlxRepository {
    type Entity = PostTypeInfo;
    type Orm = PostTypeInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostTypeInfo::from(orm)
    }
}

impl PostTypeInfoRepository for PostTypeInfoSqlxRepository {}
