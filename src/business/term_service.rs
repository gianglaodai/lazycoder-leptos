use crate::common::repository::{Repository, ViewRepository};
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

// Table entity: terms
define_struct_with_common_fields!(Term {
    req {
        pub taxonomy_id: i32,
        pub slug: String,
        pub name: String,
    }
    opt {
        pub parent_id: Option<i32>,
        pub description: Option<String>,
    }
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
}

impl<R: TermInfoRepository> ViewService for TermInfoService<R> {
    type Entity = TermInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
