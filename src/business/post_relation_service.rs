use crate::common::repository::ViewRepository;
use crate::common::service::ViewService;
use crate::define_readonly_struct_with_common_fields;
use std::sync::Arc;

define_readonly_struct_with_common_fields!(PostRelation {
    pub from_post: i32,
    pub to_post: i32,
    pub rel_type: String,
});

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
}

impl<R: PostRelationInfoRepository> ViewService for PostRelationInfoService<R> {
    type Entity = PostRelationInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
