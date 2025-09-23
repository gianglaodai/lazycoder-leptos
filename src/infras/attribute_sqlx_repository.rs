#![cfg(feature = "ssr")]

use std::collections::HashMap;
use std::future::Future;
use crate::business::attribute_service::{Attribute, AttributeCreate, AttributeRepository};
use crate::common::repository::{Repository, ViewRepository};
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxRepository, SqlxViewRepository};
use sqlx::PgPool;
use uuid::Uuid;
use crate::common::error::CoreError;
use crate::common::filter::{Filter, ScalarValue};
use crate::common::sort::SortCriterion;

#[derive(Clone)]
pub struct AttributeSqlxRepository {
    pool: PgPool,
}

define_readonly_orm_with_common_fields!(Attribute { pub name: String, pub entity_type: String, pub data_type: String, });

impl AttributeOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["name", "entity_type", "data_type"]
    }
}

impl From<AttributeOrm> for Attribute {
    fn from(orm: AttributeOrm) -> Self {
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

impl AttributeSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<Attribute> for AttributeSqlxRepository {
    fn get_table_name(&self) -> &str {
        "attributes"
    }
    fn get_columns(&self) -> Vec<&str> {
        AttributeOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        AttributeOrm::searchable_columns()
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
    ) -> Result<Vec<Attribute>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }
    async fn find_by_id(&self, id: i32) -> Result<Option<Attribute>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }
    async fn find_by_uid(&self, uid: String) -> Result<Option<Attribute>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
    async fn get_column_type_map(
        &self,
    ) -> Result<HashMap<String, ScalarValue>, CoreError>
    {
        SqlxViewRepository::get_column_type_map(self).await
    }
}

impl SqlxViewRepository for AttributeSqlxRepository {
    type Entity = Attribute;
    type Orm = AttributeOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        Attribute::from(orm)
    }
}

impl SqlxRepository for AttributeSqlxRepository {
    type EntityCreate = AttributeCreate;
}

impl Repository<Attribute, AttributeCreate> for AttributeSqlxRepository {
    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_id(self, id).await
    }

    async fn delete_by_ids(&self, ids: Vec<i32>) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_ids(self, ids).await
    }

    async fn delete_by_uid(&self, uid: String) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }

    async fn delete_by_uids(&self, uids: Vec<String>) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_uids(self, uids.iter().map(Uuid::parse_str).collect()).await
    }

    async fn create(&self, entity_create: &AttributeCreate) -> Result<Attribute, CoreError> {
        todo!()
    }

    async fn update(&self, entity: &Attribute) -> Result<Attribute, CoreError> {
        todo!()
    }

    async fn get_attribute_type_map(&self) -> Result<HashMap<String, ScalarValue>, CoreError> {
        SqlxRepository::get_attribute_type_map(self).await
    }
}

impl AttributeRepository for AttributeSqlxRepository {}
