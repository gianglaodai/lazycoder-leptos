use crate::common::error::CoreError;
use crate::common::filter::Filter;
use crate::common::repository::ViewRepository;
use crate::common::service::ViewService;
use crate::common::sort::SortCriterion;
use crate::define_readonly_struct_with_common_fields;
use std::sync::Arc;

// post_relations (table-like, read-only service due to composite PK)
define_readonly_struct_with_common_fields!(PostRelation {
    pub from_post: i32,
    pub to_post: i32,
    pub rel_type: String,
});

// post_relations_info (view)
define_readonly_struct_with_common_fields!(PostRelationInfo {
    pub from_post: i32,
    pub to_post: i32,
    pub rel_type: String,
    pub from_slug: Option<String>,
    pub from_title: Option<String>,
    pub to_slug: Option<String>,
    pub to_title: Option<String>,
});

pub trait PostRelationRepository: ViewRepository<PostRelation> + Send + Sync {}
pub trait PostRelationInfoRepository: ViewRepository<PostRelationInfo> + Send + Sync {}

#[derive(Clone)]
pub struct PostRelationService<R: PostRelationRepository> {
    repository: Arc<R>,
}

impl<R: PostRelationRepository> PostRelationService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<PostRelation>, CoreError> {
        self.repository.find_all(filters).await
    }
    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostRelation>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<PostRelation>, CoreError> {
        self.repository.find_by_id(id).await
    }
    pub async fn get_by_uid(&self, uid: String) -> Result<Option<PostRelation>, CoreError> {
        self.repository.find_by_uid(uid).await
    }
}

impl<R: PostRelationRepository> ViewService for PostRelationService<R> {
    type Entity = PostRelation;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

#[derive(Clone)]
pub struct PostRelationInfoService<R: PostRelationInfoRepository> {
    repository: Arc<R>,
}

impl<R: PostRelationInfoRepository> PostRelationInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<PostRelationInfo>, CoreError> {
        self.repository.find_all(filters).await
    }
    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostRelationInfo>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<PostRelationInfo>, CoreError> {
        self.repository.find_by_id(id).await
    }
    pub async fn get_by_uid(&self, uid: String) -> Result<Option<PostRelationInfo>, CoreError> {
        self.repository.find_by_uid(uid).await
    }
}

impl<R: PostRelationInfoRepository> ViewService for PostRelationInfoService<R> {
    type Entity = PostRelationInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
