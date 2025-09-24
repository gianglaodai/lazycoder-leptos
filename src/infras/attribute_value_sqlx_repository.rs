#![cfg(feature = "ssr")]

use crate::business::attribute_value_service::{
    AttributeValue, AttributeValueCreate, AttributeValueInfo, AttributeValueInfoRepository,
    AttributeValueRepository,
};
use crate::infras::sqlx_repository::{
    SqlxEntityMapper, SqlxRepository, SqlxViewMeta, SqlxViewRepository,
};
use crate::{define_orm_with_common_fields, define_readonly_orm_with_common_fields};
use sqlx::PgPool;
use uuid::Uuid;

define_orm_with_common_fields!(AttributeValue {
    pub attribute_id: i32,
    pub entity_id: i32,
    pub entity_type: String,
    pub int_value: Option<i32>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub date_value: Option<time::Date>,
    pub datetime_value: Option<time::OffsetDateTime>,
    pub time_value: Option<time::Time>,
});

define_readonly_orm_with_common_fields!(AttributeValueInfo {
    pub int_value: Option<i32>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub date_value: Option<time::Date>,
    pub datetime_value: Option<time::OffsetDateTime>,
    pub time_value: Option<time::Time>,
    pub attribute_id: i32,
    pub attribute_name: String,
    pub attribute_entity_type: String,
    pub attribute_data_type: String,
    pub entity_id: i32,
    pub entity_type: String,
});
impl From<AttributeValueOrm> for AttributeValue {
    fn from(orm: AttributeValueOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            int_value: orm.int_value,
            double_value: orm.double_value,
            string_value: orm.string_value,
            boolean_value: orm.boolean_value,
            date_value: orm.date_value,
            datetime_value: orm.datetime_value,
            time_value: orm.time_value,
            attribute_id: orm.attribute_id,
            entity_id: orm.entity_id,
            entity_type: orm.entity_type,
        }
    }
}
impl From<AttributeValueInfoOrm> for AttributeValueInfo {
    fn from(orm: AttributeValueInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            int_value: orm.int_value,
            double_value: orm.double_value,
            string_value: orm.string_value,
            boolean_value: orm.boolean_value,
            date_value: orm.date_value,
            datetime_value: orm.datetime_value,
            time_value: orm.time_value,
            attribute_id: orm.attribute_id,
            attribute_name: orm.attribute_name,
            attribute_entity_type: orm.attribute_entity_type,
            attribute_data_type: orm.attribute_data_type,
            entity_id: orm.entity_id,
            entity_type: orm.entity_type,
        }
    }
}

#[derive(Clone)]
pub struct AttributeValueSqlxRepository {
    pool: PgPool,
}

#[derive(Clone)]
pub struct AttributeValueInfoSqlxRepository {
    pool: PgPool,
}
impl AttributeValueOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["string_value", "entity_type"]
    }
}

impl AttributeValueSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for AttributeValueSqlxRepository {
    fn get_table_name(&self) -> &str {
        "attribute_values"
    }
    fn get_columns(&self) -> Vec<&str> {
        AttributeValueOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        AttributeValueOrm::searchable_columns()
    }
}

impl SqlxViewRepository for AttributeValueSqlxRepository {
    type Entity = AttributeValue;
    type Orm = AttributeValueOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        AttributeValue::from(orm)
    }
}

impl SqlxEntityMapper for AttributeValueSqlxRepository {
    type Entity = AttributeValue;
    type EntityCreate = AttributeValueCreate;
    type Orm = AttributeValueOrm;

    fn to_orm_from_create(&self, create: &Self::EntityCreate) -> Self::Orm {
        let now = time::OffsetDateTime::now_utc();
        AttributeValueOrm {
            id: 0,
            uid: Uuid::now_v7(),
            version: 0,
            created_at: now,
            updated_at: now,
            int_value: None,
            double_value: None,
            string_value: None,
            boolean_value: None,
            date_value: None,
            datetime_value: None,
            time_value: None,
            attribute_id: create.attribute_id,
            entity_id: create.entity_id,
            entity_type: create.entity_type.clone(),
        }
    }

    fn to_orm_from_entity(&self, entity: &Self::Entity) -> Self::Orm {
        AttributeValueOrm {
            id: entity.id,
            uid: Uuid::parse_str(&entity.uid).unwrap_or_else(|_| Uuid::nil()),
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            int_value: entity.int_value,
            double_value: entity.double_value,
            string_value: entity.string_value.clone(),
            boolean_value: entity.boolean_value,
            date_value: entity.date_value,
            datetime_value: entity.datetime_value,
            time_value: entity.time_value,
            attribute_id: entity.attribute_id,
            entity_id: entity.entity_id,
            entity_type: entity.entity_type.clone(),
        }
    }
}

impl SqlxRepository for AttributeValueSqlxRepository {
    type EntityCreate = AttributeValueCreate;
}

impl AttributeValueRepository for AttributeValueSqlxRepository {}

impl AttributeValueInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec![
            "string_value",
            "attribute_name",
            "attribute_entity_type",
            "attribute_data_type",
            "entity_type",
        ]
    }
}

impl AttributeValueInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for AttributeValueInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "attribute_values_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        AttributeValueInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        AttributeValueInfoOrm::searchable_columns()
    }
}

impl SqlxViewRepository for AttributeValueInfoSqlxRepository {
    type Entity = AttributeValueInfo;
    type Orm = AttributeValueInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        AttributeValueInfo::from(orm)
    }
}

impl AttributeValueInfoRepository for AttributeValueInfoSqlxRepository {}
