use crate::common::error::CoreError;
use crate::common::filter::Filter;
use crate::common::repository::{Repository, ViewRepository};
use crate::common::service::{Service, ViewService};
use crate::common::sort::SortCriterion;
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

// Table entity: post_collections
define_struct_with_common_fields!(PostCollection {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub visibility: String,
});

define_readonly_struct_with_common_fields!(PostCollectionInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub visibility: String,
});

pub trait PostCollectionRepository:
    Repository<PostCollection, PostCollectionCreate> + Send + Sync
{
}
pub trait PostCollectionInfoRepository: ViewRepository<PostCollectionInfo> + Send + Sync {}

// Service for table (CRUD)
#[derive(Clone)]
pub struct PostCollectionService<R: PostCollectionRepository> {
    repository: Arc<R>,
}

impl<R: PostCollectionRepository> PostCollectionService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<PostCollection>, CoreError> {
        self.repository.find_all(filters).await
    }

    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostCollection>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }

    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<PostCollection>, CoreError> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_by_uid(&self, uid: String) -> Result<Option<PostCollection>, CoreError> {
        self.repository.find_by_uid(uid).await
    }

    pub async fn create(&self, create: &PostCollectionCreate) -> Result<PostCollection, CoreError> {
        self.repository.create(create).await
    }

    pub async fn update(&self, entity: &PostCollection) -> Result<PostCollection, CoreError> {
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

impl<R: PostCollectionRepository> ViewService for PostCollectionService<R> {
    type Entity = PostCollection;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

impl<R: PostCollectionRepository> Service for PostCollectionService<R> {
    type Create = PostCollectionCreate;
}

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

impl<R: PostCollectionInfoRepository> ViewService for PostCollectionInfoService<R> {
    type Entity = PostCollectionInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
