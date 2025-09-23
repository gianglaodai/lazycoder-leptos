use crate::common::error::CoreError;
use crate::common::filter::Filter;
use crate::common::repository::{Repository, ViewRepository};
use crate::common::service::{Service, ViewService};
use crate::common::sort::SortCriterion;
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

// Table entity: terms
define_struct_with_common_fields!(Term {
    pub taxonomy_id: i32,
    pub parent_id: Option<i32>,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
});

// View entity: enriched term info
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

// Repositories
pub trait TermRepository: Repository<Term, TermCreate> + Send + Sync {}
pub trait TermInfoRepository: ViewRepository<TermInfo> + Send + Sync {}

// Services for table (CRUD)
#[derive(Clone)]
pub struct TermService<R: TermRepository> {
    repository: Arc<R>,
}

impl<R: TermRepository> TermService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<Term>, CoreError> {
        self.repository.find_all(filters).await
    }

    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<Term>, CoreError> {
        self.repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }

    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.repository.count(filters).await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<Term>, CoreError> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_by_uid(&self, uid: String) -> Result<Option<Term>, CoreError> {
        self.repository.find_by_uid(uid).await
    }

    pub async fn create(&self, create: &TermCreate) -> Result<Term, CoreError> {
        self.repository.create(create).await
    }

    pub async fn update(&self, entity: &Term) -> Result<Term, CoreError> {
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

impl<R: TermRepository> ViewService for TermService<R> {
    type Entity = Term;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

impl<R: TermRepository> Service for TermService<R> {
    type Create = TermCreate;
}

// Service for view (read-only)
#[derive(Clone)]
pub struct TermInfoService<R: TermInfoRepository> {
    repository: Arc<R>,
}

impl<R: TermInfoRepository> TermInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<TermInfo>, CoreError> {
        self.repository.find_all(filters).await
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

impl<R: TermInfoRepository> ViewService for TermInfoService<R> {
    type Entity = TermInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
