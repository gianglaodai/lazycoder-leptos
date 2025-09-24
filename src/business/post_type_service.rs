use crate::common::repository::{Repository, ViewRepository};
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

define_struct_with_common_fields!(PostType {
    req {
        pub code: String,
        pub name: String,
    }
    opt {}
});

define_readonly_struct_with_common_fields!(PostTypeInfo {
    pub code: String,
    pub name: String,
});

// Repositories
pub trait PostTypeRepository: Repository<PostType, PostTypeCreate> + Send + Sync {}
pub trait PostTypeInfoRepository: ViewRepository<PostTypeInfo> + Send + Sync {}

// Services
#[derive(Clone)]
pub struct PostTypeService<R: PostTypeRepository> {
    repository: Arc<R>,
}

impl<R: PostTypeRepository> PostTypeService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: PostTypeRepository> ViewService for PostTypeService<R> {
    type Entity = PostType;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

impl<R: PostTypeRepository> Service for PostTypeService<R> {
    type Create = PostTypeCreate;
}

#[derive(Clone)]
pub struct PostTypeInfoService<R: PostTypeInfoRepository> {
    repository: Arc<R>,
}

impl<R: PostTypeInfoRepository> PostTypeInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: PostTypeInfoRepository> ViewService for PostTypeInfoService<R> {
    type Entity = PostTypeInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
