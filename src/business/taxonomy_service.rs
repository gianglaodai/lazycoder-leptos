use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_readonly_struct_with_common_fields;
use std::sync::Arc;

// post_taxonomies_info

define_readonly_struct_with_common_fields!(PostTaxonomyInfo {
    pub code: String,
    pub name: String,
});

pub trait PostTaxonomyInfoRepository: ViewRepository<PostTaxonomyInfo> + Send + Sync {}

#[derive(Clone)]
pub struct PostTaxonomyInfoService<R: PostTaxonomyInfoRepository> {
    repository: Arc<R>,
}

impl<R: PostTaxonomyInfoRepository> PostTaxonomyInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostTaxonomyInfo>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<PostTaxonomyInfo>, CoreError> {
        self.repository.find_by_id(id).await
    }
    pub async fn get_by_uid(&self, uid: String) -> Result<Option<PostTaxonomyInfo>, CoreError> {
        self.repository.find_by_uid(uid).await
    }
}

// terms_info

define_readonly_struct_with_common_fields!(TermInfo {
    pub taxonomy_id: i32,
    pub taxonomy_code: String,
    pub taxonomy_name: String,
    pub parent_id: Option<i32>,
    pub parent_slug: Option<String>,
    pub parent_name: Option<String>,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
});

pub trait TermInfoRepository: ViewRepository<TermInfo> + Send + Sync {}

#[derive(Clone)]
pub struct TermInfoService<R: TermInfoRepository> {
    repository: Arc<R>,
}

impl<R: TermInfoRepository> TermInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<TermInfo>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<TermInfo>, CoreError> {
        self.repository.find_by_id(id).await
    }
    pub async fn get_by_uid(&self, uid: String) -> Result<Option<TermInfo>, CoreError> {
        self.repository.find_by_uid(uid).await
    }
}
