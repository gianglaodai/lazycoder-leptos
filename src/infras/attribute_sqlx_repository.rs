#![cfg(feature = "ssr")]
use crate::business::attribute_service::{Attribute, AttributeRepository};
use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxViewRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AttributeSqlxRepository { pool: PgPool }

define_readonly_orm_with_common_fields!(Attribute { pub name: String, pub entity_type: String, pub data_type: String, });

impl AttributeOrm { pub fn searchable_columns() -> Vec<&'static str> { vec!["name", "entity_type", "data_type"] } }

impl From<AttributeOrm> for Attribute {
    fn from(orm: AttributeOrm) -> Self {
        Self { id: orm.id, uid: orm.uid.to_string(), version: orm.version, created_at: orm.created_at, updated_at: orm.updated_at, name: orm.name, entity_type: orm.entity_type, data_type: orm.data_type }
    }
}

impl AttributeSqlxRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

impl ViewRepository<Attribute> for AttributeSqlxRepository {
    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> { SqlxViewRepository::count(self, filters).await }
    async fn find_many(&self, sort_criteria: Vec<SortCriterion>, first_result: Option<i32>, max_results: Option<i32>, filters: Vec<Filter>) -> Result<Vec<Attribute>, CoreError> { SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await }
    async fn find_by_id(&self, id: i32) -> Result<Option<Attribute>, CoreError> { SqlxViewRepository::find_by_id(self, id).await }
    async fn find_by_uid(&self, uid: String) -> Result<Option<Attribute>, CoreError> { SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await }
}

impl SqlxViewRepository for AttributeSqlxRepository {
    type Entity = Attribute; type Orm = AttributeOrm;
    fn get_table_name(&self) -> &str { "attributes" }
    fn get_columns(&self) -> Vec<&'static str> { AttributeOrm::columns() }
    fn get_searchable_columns(&self) -> Vec<&str> { AttributeOrm::searchable_columns() }
    fn get_pool(&self) -> &PgPool { &self.pool }
    fn from_orm(orm: Self::Orm) -> Self::Entity { Attribute::from(orm) }
}

impl AttributeRepository for AttributeSqlxRepository {}
