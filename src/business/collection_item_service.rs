use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::ViewRepository;
use crate::business::sort::SortCriterion;
use crate::define_readonly_struct_with_common_fields;
use std::sync::Arc;

// post_collection_items_info

define_readonly_struct_with_common_fields!(PostCollectionItemInfo {
    pub post_collection_id: i32,
    pub post_id: i32,
    pub position: i32,
    pub headline: Option<String>,
    pub collection_slug: Option<String>,
    pub collection_title: Option<String>,
    pub post_slug: Option<String>,
    pub post_title: Option<String>,
});

pub trait PostCollectionItemInfoRepository:
    ViewRepository<PostCollectionItemInfo> + Send + Sync
{
}

#[derive(Clone)]
pub struct PostCollectionItemInfoService<R: PostCollectionItemInfoRepository> {
    repository: Arc<R>,
}

impl<R: PostCollectionItemInfoRepository> PostCollectionItemInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostCollectionItemInfo>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<PostCollectionItemInfo>, CoreError> {
        self.repository.find_by_id(id).await
    }
    pub async fn get_by_uid(
        &self,
        uid: String,
    ) -> Result<Option<PostCollectionItemInfo>, CoreError> {
        self.repository.find_by_uid(uid).await
    }
}

impl<R: PostCollectionItemInfoRepository> crate::business::service::ViewService
    for PostCollectionItemInfoService<R>
{
    type Entity = PostCollectionItemInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
