#![cfg(feature = "ssr")]

use std::collections::HashMap;
use crate::common::repository::ViewRepository;
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxViewRepository;
use sqlx::PgPool;
use uuid::Uuid;
use crate::business::attribute_service::{AttributeInfo, AttributeInfoRepository};
use crate::common::error::CoreError;
use crate::common::filter::{Filter, ScalarValue};
use crate::common::sort::SortCriterion;

#[derive(Clone)]
pub struct AttributeInfoSqlxRepository {
    pool: PgPool,
}

define_readonly_orm_with_common_fields!(AttributeInfo { pub name: String, pub entity_type: String, pub data_type: String, });

impl AttributeInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["name", "entity_type", "data_type"]
    }
}

impl From<AttributeInfoOrm> for AttributeInfo {
    fn from(orm: AttributeInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            name: orm.name,
            entity_type: orm.entity_type,
            data_type: orm.data_type,
        }
    }
}

impl AttributeInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<AttributeInfo> for AttributeInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "attributes_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        AttributeInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        AttributeInfoOrm::searchable_columns()
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
    ) -> Result<Vec<AttributeInfo>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }
    async fn find_by_id(&self, id: i32) -> Result<Option<AttributeInfo>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }
    async fn find_by_uid(&self, uid: String) -> Result<Option<AttributeInfo>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
    async fn get_column_type_map(
        &self,
    ) -> Result<HashMap<String, ScalarValue>, CoreError>
    {
        SqlxViewRepository::get_column_type_map(self).await
    }
}

impl SqlxViewRepository for AttributeInfoSqlxRepository {
    type Entity = AttributeInfo;
    type Orm = AttributeInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        AttributeInfo::from(orm)
    }
}

impl AttributeInfoRepository for AttributeInfoSqlxRepository {}
