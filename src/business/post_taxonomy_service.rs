use crate::common::error::CoreError;
use crate::common::filter::Filter;
use crate::common::repository::{Repository, ViewRepository};
use crate::common::service::{Service, ViewService};
use crate::common::sort::SortCriterion;
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

define_struct_with_common_fields!(PostTaxonomy {
    pub code: String,
    pub name: String,
});

define_readonly_struct_with_common_fields!(PostTaxonomyInfo {
    pub code: String,
    pub name: String,
});

pub trait PostTaxonomyRepository:
    Repository<PostTaxonomy, PostTaxonomyCreate> + Send + Sync
{
}
pub trait PostTaxonomyInfoRepository: ViewRepository<PostTaxonomyInfo> + Send + Sync {}

#[derive(Clone)]
pub struct PostTaxonomyService<R: PostTaxonomyRepository> {
    repository: Arc<R>,
}

impl<R: PostTaxonomyRepository> PostTaxonomyService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<PostTaxonomy>, CoreError> {
        self.repository.find_all(filters).await
    }

    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostTaxonomy>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }

    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<PostTaxonomy>, CoreError> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_by_uid(&self, uid: String) -> Result<Option<PostTaxonomy>, CoreError> {
        self.repository.find_by_uid(uid).await
    }

    pub async fn create(&self, create: &PostTaxonomyCreate) -> Result<PostTaxonomy, CoreError> {
        self.repository.create(create).await
    }

    pub async fn update(&self, entity: &PostTaxonomy) -> Result<PostTaxonomy, CoreError> {
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

impl<R: PostTaxonomyRepository> ViewService for PostTaxonomyService<R> {
    type Entity = PostTaxonomy;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

impl<R: PostTaxonomyRepository> Service for PostTaxonomyService<R> {
    type Create = PostTaxonomyCreate;
}

#[derive(Clone)]
pub struct PostTaxonomyInfoService<R: PostTaxonomyInfoRepository> {
    repository: Arc<R>,
}

impl<R: PostTaxonomyInfoRepository> PostTaxonomyInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<PostTaxonomyInfo>, CoreError> {
        self.repository.find_all(filters).await
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

impl<R: PostTaxonomyInfoRepository> ViewService for PostTaxonomyInfoService<R> {
    type Entity = PostTaxonomyInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
