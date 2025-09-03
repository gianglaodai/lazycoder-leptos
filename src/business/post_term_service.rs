use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_readonly_struct_with_common_fields;
use std::sync::Arc;

// post_terms_info

define_readonly_struct_with_common_fields!(PostTermInfo {
    pub post_id: i32,
    pub term_id: i32,
    pub post_slug: Option<String>,
    pub post_title: Option<String>,
    pub term_slug: Option<String>,
    pub term_name: Option<String>,
    pub taxonomy_id: Option<i32>,
    pub taxonomy_code: Option<String>,
});

pub trait PostTermInfoRepository: ViewRepository<PostTermInfo> + Send + Sync {}

#[derive(Clone)]
pub struct PostTermInfoService<R: PostTermInfoRepository> { repository: Arc<R> }

impl<R: PostTermInfoRepository> PostTermInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self { Self { repository } }
    pub async fn get_many(&self, sort_criteria: Vec<SortCriterion>, first_result: Option<i32>, max_results: Option<i32>, filters: Vec<Filter>) -> Result<Vec<PostTermInfo>, CoreError> { self.repository.find_many(sort_criteria, first_result, max_results, filters).await }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> { self.repository.count(filters).await }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<PostTermInfo>, CoreError> { self.repository.find_by_id(id).await }
    pub async fn get_by_uid(&self, uid: String) -> Result<Option<PostTermInfo>, CoreError> { self.repository.find_by_uid(uid).await }
}
