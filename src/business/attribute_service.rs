use crate::common::error::CoreError;
use crate::common::filter::Filter;
use crate::common::repository::ViewRepository;
use crate::common::service::ViewService;
use crate::common::sort::SortCriterion;
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

define_struct_with_common_fields!(Attribute {
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
    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<Attribute>, CoreError> {
        self.repository.find_all(filters).await
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

impl<R: AttributeRepository> ViewService for AttributeService<R> {
    type Entity = Attribute;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

// View entity: attributes_info (readonly)
define_readonly_struct_with_common_fields!(AttributeInfo {
    pub name: String,
    pub entity_type: String,
    pub data_type: String,
});

pub trait AttributeInfoRepository: ViewRepository<AttributeInfo> + Send + Sync {}

#[derive(Clone)]
pub struct AttributeInfoService<R: AttributeInfoRepository> {
    repository: Arc<R>,
}

impl<R: AttributeInfoRepository> AttributeInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<AttributeInfo>, CoreError> {
        self.repository.find_all(filters).await
    }
    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<AttributeInfo>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<AttributeInfo>, CoreError> {
        self.repository.find_by_id(id).await
    }
    pub async fn get_by_uid(&self, uid: String) -> Result<Option<AttributeInfo>, CoreError> {
        self.repository.find_by_uid(uid).await
    }
}

impl<R: AttributeInfoRepository> ViewService for AttributeInfoService<R> {
    type Entity = AttributeInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
