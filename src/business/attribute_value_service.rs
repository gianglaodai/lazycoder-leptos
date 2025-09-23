use crate::common::error::CoreError;
use crate::common::filter::Filter;
use crate::common::repository::{Repository, ViewRepository};
use crate::common::service::{Service, ViewService};
use crate::common::sort::SortCriterion;
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

// Table entity: attribute_values
define_struct_with_common_fields!(AttributeValue {
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

// View entity: attribute_values with joined attribute metadata
define_readonly_struct_with_common_fields!(AttributeValueInfo {
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

// Repositories
pub trait AttributeValueRepository:
    Repository<AttributeValue, AttributeValueCreate> + Send + Sync
{
}
pub trait AttributeValueInfoRepository: ViewRepository<AttributeValueInfo> + Send + Sync {}

// Service for table (CRUD)
#[derive(Clone)]
pub struct AttributeValueService<R: AttributeValueRepository> {
    repository: Arc<R>,
}

impl<R: AttributeValueRepository> AttributeValueService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<AttributeValue>, CoreError> {
        self.repository.find_all(filters).await
    }

    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<AttributeValue>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }

    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<AttributeValue>, CoreError> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_by_uid(&self, uid: String) -> Result<Option<AttributeValue>, CoreError> {
        self.repository.find_by_uid(uid).await
    }

    pub async fn create(&self, create: &AttributeValueCreate) -> Result<AttributeValue, CoreError> {
        self.repository.create(create).await
    }

    pub async fn update(&self, entity: &AttributeValue) -> Result<AttributeValue, CoreError> {
        self.repository.update(entity).await
    }

    pub async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        self.repository.delete_by_id(id).await
    }

    pub async fn delete_by_ids(&self, ids: Vec<i32>) -> Result<u64, CoreError> {
        self.repository.delete_by_ids(ids).await
    }

    pub async fn delete_by_uid(&self, uid: String) -> Result<u64, CoreError> {
        self.repository.delete_by_uid(uid).await
    }
}

impl<R: AttributeValueRepository> ViewService for AttributeValueService<R> {
    type Entity = AttributeValue;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

impl<R: AttributeValueRepository> Service for AttributeValueService<R> {
    type Create = AttributeValueCreate;
}

// Service for view (read-only)
#[derive(Clone)]
pub struct AttributeValueInfoService<R: AttributeValueInfoRepository> {
    repository: Arc<R>,
}

impl<R: AttributeValueInfoRepository> AttributeValueInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    pub async fn get_all(
        &self,
        filters: Vec<Filter>,
    ) -> Result<Vec<AttributeValueInfo>, CoreError> {
        self.repository.find_all(filters).await
    }
    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<AttributeValueInfo>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<AttributeValueInfo>, CoreError> {
        self.repository.find_by_id(id).await
    }
    pub async fn get_by_uid(&self, uid: String) -> Result<Option<AttributeValueInfo>, CoreError> {
        self.repository.find_by_uid(uid).await
    }
}

impl<R: AttributeValueInfoRepository> ViewService for AttributeValueInfoService<R> {
    type Entity = AttributeValueInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
