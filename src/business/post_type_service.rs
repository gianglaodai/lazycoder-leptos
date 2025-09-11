use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::{Repository, ViewRepository};
use crate::business::sort::SortCriterion;
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

define_struct_with_common_fields!(PostType {
    pub code: String,
    pub name: String,
});

define_readonly_struct_with_common_fields!(PostTypeInfo {
    pub code: String,
    pub name: String,
});

// Repositories
pub trait PostTypeRepository: Repository<PostType, PostTypeCreate> + Send + Sync {}
pub trait PostTypeInfoRepository: ViewRepository<PostTypeInfo> + Send + Sync {}

// Services
#[derive(Clone)]
pub struct PostTypeService<R: PostTypeRepository> {
    repository: Arc<R>,
}

impl<R: PostTypeRepository> PostTypeService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<PostType>, CoreError> {
        self.repository.find_all(filters).await
    }

    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostType>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }

    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<PostType>, CoreError> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_by_uid(&self, uid: String) -> Result<Option<PostType>, CoreError> {
        self.repository.find_by_uid(uid).await
    }

    pub async fn create(&self, create: &PostTypeCreate) -> Result<PostType, CoreError> {
        self.repository.create(create).await
    }

    pub async fn update(&self, entity: &PostType) -> Result<PostType, CoreError> {
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

#[derive(Clone)]
pub struct PostTypeInfoService<R: PostTypeInfoRepository> {
    repository: Arc<R>,
}

impl<R: PostTypeInfoRepository> PostTypeInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<PostTypeInfo>, CoreError> {
        self.repository.find_all(filters).await
    }

    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostTypeInfo>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }

    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<PostTypeInfo>, CoreError> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_by_uid(&self, uid: String) -> Result<Option<PostTypeInfo>, CoreError> {
        self.repository.find_by_uid(uid).await
    }
}
