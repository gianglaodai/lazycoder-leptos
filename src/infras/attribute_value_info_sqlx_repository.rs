#![cfg(feature = "ssr")]
use crate::business::attribute_service::{AttributeValueInfo, AttributeValueInfoRepository};
use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxViewRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AttributeValueInfoSqlxRepository {
    pool: PgPool,
}

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

impl AttributeValueInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<AttributeValueInfo> for AttributeValueInfoSqlxRepository {
    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        SqlxViewRepository::count(self, filters).await
    }
    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<AttributeValueInfo>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }
    async fn find_by_id(&self, id: i32) -> Result<Option<AttributeValueInfo>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }
    async fn find_by_uid(&self, uid: String) -> Result<Option<AttributeValueInfo>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
}

impl SqlxViewRepository for AttributeValueInfoSqlxRepository {
    type Entity = AttributeValueInfo;
    type Orm = AttributeValueInfoOrm;
    fn get_table_name(&self) -> &str {
        "attribute_values_info"
    }
    fn get_columns(&self) -> Vec<&'static str> {
        AttributeValueInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        AttributeValueInfoOrm::searchable_columns()
    }
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        AttributeValueInfo::from(orm)
    }
}

impl AttributeValueInfoRepository for AttributeValueInfoSqlxRepository {}
