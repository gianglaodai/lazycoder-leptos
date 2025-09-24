#![cfg(feature = "ssr")]

use crate::business::attribute_service::{
    Attribute, AttributeInfo, AttributeInfoRepository, AttributeRepository,
};
use crate::infras::sqlx_repository::{SqlxViewMeta, SqlxViewRepository};
use crate::{define_orm_with_common_fields, define_readonly_orm_with_common_fields};
use sqlx::PgPool;

define_orm_with_common_fields!(Attribute {
    pub name: String,
    pub entity_type: String,
    pub data_type: String,
});

define_readonly_orm_with_common_fields!(AttributeInfo {
    pub name: String,
    pub entity_type: String,
    pub data_type: String,
});
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

#[derive(Clone)]
pub struct AttributeSqlxRepository {
    pool: PgPool,
}
#[derive(Clone)]
pub struct AttributeInfoSqlxRepository {
    pool: PgPool,
}
impl AttributeOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["name", "entity_type", "data_type"]
    }
}

impl AttributeSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for AttributeSqlxRepository {
    fn get_table_name(&self) -> &str {
        "attributes"
    }
    fn get_columns(&self) -> Vec<&str> {
        AttributeOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        AttributeOrm::searchable_columns()
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

impl AttributeRepository for AttributeSqlxRepository {}

impl AttributeInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["name", "entity_type", "data_type"]
    }
}

impl AttributeInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for AttributeInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "attributes_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        AttributeInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        AttributeInfoOrm::searchable_columns()
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
