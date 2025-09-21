use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_readonly_struct_with_common_fields;
use std::sync::Arc;

// attributes (table) as info

define_readonly_struct_with_common_fields!(Attribute {
    pub name: String,
    pub entity_type: String,
    pub data_type: String,
});

pub trait AttributeRepository: ViewRepository<Attribute> + Send + Sync {}

#[derive(Clone)]
pub struct AttributeService<R: AttributeRepository> {
    repository: Arc<R>,
}

impl<R: AttributeRepository> AttributeService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<Attribute>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<Attribute>, CoreError> {
        self.repository.find_by_id(id).await
    }
    pub async fn get_by_uid(&self, uid: String) -> Result<Option<Attribute>, CoreError> {
        self.repository.find_by_uid(uid).await
    }
}

impl<R: AttributeRepository> crate::business::service::ViewService for AttributeService<R> {
    type Entity = Attribute;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

// attribute_values_info

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

pub trait AttributeValueInfoRepository: ViewRepository<AttributeValueInfo> + Send + Sync {}

#[derive(Clone)]
pub struct AttributeValueInfoService<R: AttributeValueInfoRepository> {
    repository: Arc<R>,
}

impl<R: AttributeValueInfoRepository> AttributeValueInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
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

impl<R: AttributeValueInfoRepository> crate::business::service::ViewService
    for AttributeValueInfoService<R>
{
    type Entity = AttributeValueInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
