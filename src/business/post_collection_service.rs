use crate::common::repository::{Repository, ViewRepository};
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

// Table entity: post_collections
define_struct_with_common_fields!(PostCollection {
    req {
        pub slug: String,
        pub title: String,
        pub visibility: String,
    }
    opt {
        pub description: Option<String>,
    }
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
}

impl<R: PostCollectionInfoRepository> ViewService for PostCollectionInfoService<R> {
    type Entity = PostCollectionInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
