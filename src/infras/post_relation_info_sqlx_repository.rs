#![cfg(feature = "ssr")]

use std::collections::HashMap;
use crate::business::post_relation_service::{PostRelationInfo, PostRelationInfoRepository};
use crate::common::repository::ViewRepository;
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxViewRepository;
use sqlx::PgPool;
use uuid::Uuid;
use crate::common::error::CoreError;
use crate::common::filter::{Filter, ScalarValue};
use crate::common::sort::SortCriterion;

#[derive(Clone)]
pub struct PostRelationInfoSqlxRepository {
    pool: PgPool,
}

define_readonly_orm_with_common_fields!(PostRelationInfo {
    pub from_post: i32,
    pub to_post: i32,
    pub rel_type: String,
    pub from_slug: Option<String>,
    pub from_title: Option<String>,
    pub to_slug: Option<String>,
    pub to_title: Option<String>,
});

impl PostRelationInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["rel_type", "from_slug", "to_slug", "from_title", "to_title"]
    }
}

impl From<PostRelationInfoOrm> for PostRelationInfo {
    fn from(orm: PostRelationInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            from_post: orm.from_post,
            to_post: orm.to_post,
            rel_type: orm.rel_type,
            from_slug: orm.from_slug,
            from_title: orm.from_title,
            to_slug: orm.to_slug,
            to_title: orm.to_title,
        }
    }
}

impl PostRelationInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<PostRelationInfo> for PostRelationInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_relations_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostRelationInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostRelationInfoOrm::searchable_columns()
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
    ) -> Result<Vec<PostRelationInfo>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }
    async fn find_by_id(&self, id: i32) -> Result<Option<PostRelationInfo>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }
    async fn find_by_uid(&self, uid: String) -> Result<Option<PostRelationInfo>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
    async fn get_column_type_map(
        &self,
    ) -> Result<HashMap<String, ScalarValue>, CoreError>
    {
        SqlxViewRepository::get_column_type_map(self).await
    }
}

impl SqlxViewRepository for PostRelationInfoSqlxRepository {
    type Entity = PostRelationInfo;
    type Orm = PostRelationInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostRelationInfo::from(orm)
    }
}

impl PostRelationInfoRepository for PostRelationInfoSqlxRepository {}
