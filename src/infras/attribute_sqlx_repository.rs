#![cfg(feature = "ssr")]

use crate::business::attribute_service::{
    Attribute, AttributeCreate, AttributeInfo, AttributeInfoRepository, AttributeRepository,
};
use crate::infras::sqlx_repository::{
    SqlxEntityMapper, SqlxRepository, SqlxViewMeta, SqlxViewRepository,
};
use crate::{define_orm_with_common_fields, define_readonly_orm_with_common_fields};
use sqlx::PgPool;
use uuid::Uuid;

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

impl SqlxEntityMapper for AttributeSqlxRepository {
    type Entity = Attribute;
    type EntityCreate = AttributeCreate;
    type Orm = AttributeOrm;

    fn to_orm_from_create(&self, create: &Self::EntityCreate) -> Self::Orm {
        let now = time::OffsetDateTime::now_utc();
        AttributeOrm {
            id: 0,
            uid: Uuid::now_v7(),
            version: 0,
            created_at: now,
            updated_at: now,
            name: create.name.clone(),
            entity_type: create.entity_type.clone(),
            data_type: create.data_type.clone(),
        }
    }

    fn to_orm_from_entity(&self, entity: &Self::Entity) -> Self::Orm {
        AttributeOrm {
            id: entity.id,
            uid: Uuid::parse_str(&entity.uid).unwrap_or_else(|_| Uuid::nil()),
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            name: entity.name.clone(),
            entity_type: entity.entity_type.clone(),
            data_type: entity.data_type.clone(),
        }
    }
}

impl SqlxRepository for AttributeSqlxRepository {
    type EntityCreate = AttributeCreate;
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
