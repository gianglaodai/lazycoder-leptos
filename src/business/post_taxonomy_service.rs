use crate::common::repository::{Repository, ViewRepository};
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

define_struct_with_common_fields!(PostTaxonomy {
    req {
        pub code: String,
        pub name: String,
    }
    opt {}
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
}

impl<R: PostTaxonomyInfoRepository> ViewService for PostTaxonomyInfoService<R> {
    type Entity = PostTaxonomyInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
