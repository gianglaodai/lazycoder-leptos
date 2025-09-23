#![cfg(feature = "ssr")]

use crate::business::attribute_value_service::{
    AttributeValue, AttributeValueCreate, AttributeValueRepository,
};
use crate::common::error::CoreError;
use crate::common::filter::{Filter, ScalarValue};
use crate::common::repository::{Repository, ViewRepository};
use crate::common::sort::SortCriterion;
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxRepository, SqlxViewRepository};
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::collections::HashMap;
use std::future::Future;
use uuid::Uuid;

#[derive(Clone)]
pub struct AttributeValueSqlxRepository {
    pool: PgPool,
}

// ORM for table attribute_values
define_orm_with_common_fields!(AttributeValue {
    pub int_value: Option<i32>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub date_value: Option<time::Date>,
    pub datetime_value: Option<time::OffsetDateTime>,
    pub time_value: Option<time::Time>,
    pub attribute_id: i32,
    pub entity_id: i32,
    pub entity_type: String,
});

impl AttributeValueOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["string_value", "entity_type"]
    }
}

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

impl AttributeValueSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<AttributeValue> for AttributeValueSqlxRepository {
    fn get_table_name(&self) -> &str {
        "attribute_values"
    }
    fn get_columns(&self) -> Vec<&str> {
        AttributeValueOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        AttributeValueOrm::searchable_columns()
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
    ) -> Result<Vec<AttributeValue>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }
    async fn find_by_id(&self, id: i32) -> Result<Option<AttributeValue>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }
    async fn find_by_uid(&self, uid: String) -> Result<Option<AttributeValue>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
    async fn get_column_type_map(&self) -> Result<HashMap<String, ScalarValue>, CoreError> {
        SqlxViewRepository::get_column_type_map(self).await
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

impl SqlxRepository for AttributeValueSqlxRepository {
    type EntityCreate = AttributeValueCreate;
}

impl Repository<AttributeValue, AttributeValueCreate> for AttributeValueSqlxRepository {
    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_id(self, id)
    }
    async fn delete_by_ids(&self, ids: Vec<i32>) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_ids(self, ids)
    }
    async fn delete_by_uid(&self, uid: String) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_uid(self, Uuid::parse_str(&uid).unwrap())
    }

    async fn delete_by_uids(&self, uids: Vec<String>) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_uids(self, uids.iter().map(Uuid::parse_str).collect())
    }

    async fn create(&self, create: &AttributeValueCreate) -> Result<AttributeValue, CoreError> {
        let pool = self.pool.clone();
        let create = create.clone();
        async move {
            let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
                "INSERT INTO attribute_values (uid, version, int_value, double_value, string_value, boolean_value, date_value, datetime_value, time_value, attribute_id, entity_id, entity_type) ",
            );
            qb.push("VALUES (")
                .push_bind(Uuid::new_v4())
                .push(", ")
                .push_bind(0i32)
                .push(", ")
                .push_bind(create.int_value)
                .push(", ")
                .push_bind(create.double_value)
                .push(", ")
                .push_bind(create.string_value)
                .push(", ")
                .push_bind(create.boolean_value)
                .push(", ")
                .push_bind(create.date_value)
                .push(", ")
                .push_bind(create.datetime_value)
                .push(", ")
                .push_bind(create.time_value)
                .push(", ")
                .push_bind(create.attribute_id)
                .push(", ")
                .push_bind(create.entity_id)
                .push(", ")
                .push_bind(create.entity_type)
                .push(") RETURNING *");

            let row = qb.build().fetch_one(&pool).await?;
            let orm: AttributeValueOrm = sqlx::FromRow::from_row(&row)?;
            Ok(AttributeValue::from(orm))
        }
    }

    async fn update(&self, entity: &AttributeValue) -> Result<AttributeValue, CoreError> {
        let pool = self.pool.clone();
        let e = entity.clone();
        async move {
            let mut qb: QueryBuilder<Postgres> =
                QueryBuilder::new("UPDATE attribute_values SET version = ");
            qb.push_bind(e.version + 1)
                .push(", int_value = ")
                .push_bind(e.int_value)
                .push(", double_value = ")
                .push_bind(e.double_value)
                .push(", string_value = ")
                .push_bind(e.string_value)
                .push(", boolean_value = ")
                .push_bind(e.boolean_value)
                .push(", date_value = ")
                .push_bind(e.date_value)
                .push(", datetime_value = ")
                .push_bind(e.datetime_value)
                .push(", time_value = ")
                .push_bind(e.time_value)
                .push(", attribute_id = ")
                .push_bind(e.attribute_id)
                .push(", entity_id = ")
                .push_bind(e.entity_id)
                .push(", entity_type = ")
                .push_bind(e.entity_type)
                .push(" WHERE id = ")
                .push_bind(e.id)
                .push(" RETURNING *");

            let row = qb.build().fetch_one(&pool).await?;
            let orm: AttributeValueOrm = sqlx::FromRow::from_row(&row)?;
            Ok(AttributeValue::from(orm))
        }
    }

    async fn get_attribute_type_map(&self) -> Result<HashMap<String, ScalarValue>, CoreError> {
        SqlxRepository::get_attribute_type_map(self)
    }
}

impl AttributeValueRepository for AttributeValueSqlxRepository {}
