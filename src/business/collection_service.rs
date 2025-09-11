use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_readonly_struct_with_common_fields;
use std::sync::Arc;

define_readonly_struct_with_common_fields!(PostCollectionInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub visibility: String,
});

pub trait PostCollectionInfoRepository: ViewRepository<PostCollectionInfo> + Send + Sync {}

#[derive(Clone)]
pub struct PostCollectionInfoService<R: PostCollectionInfoRepository> {
    repository: Arc<R>,
}

impl<R: PostCollectionInfoRepository> PostCollectionInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_all(
        &self,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostCollectionInfo>, CoreError> {
        self.repository.find_all(filters).await
    }

    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostCollectionInfo>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }

    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<PostCollectionInfo>, CoreError> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_by_uid(&self, uid: String) -> Result<Option<PostCollectionInfo>, CoreError> {
        self.repository.find_by_uid(uid).await
    }
}
