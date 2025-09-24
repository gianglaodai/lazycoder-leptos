use crate::common::repository::ViewRepository;
use crate::common::service::ViewService;
use crate::define_readonly_struct_with_common_fields;
use std::sync::Arc;

define_readonly_struct_with_common_fields!(PostTerm {
    pub post_id: i32,
    pub term_id: i32,
});

define_readonly_struct_with_common_fields!(PostTermInfo {
    pub post_id: i32,
    pub term_id: i32,
    pub post_slug: Option<String>,
    pub post_title: Option<String>,
    pub term_slug: Option<String>,
    pub term_name: Option<String>,
    pub taxonomy_id: Option<i32>,
    pub taxonomy_code: Option<String>,
});

pub trait PostTermRepository: ViewRepository<PostTerm> + Send + Sync {}
pub trait PostTermInfoRepository: ViewRepository<PostTermInfo> + Send + Sync {}

#[derive(Clone)]
pub struct PostTermService<R: PostTermRepository> {
    repository: Arc<R>,
}

impl<R: PostTermRepository> PostTermService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: PostTermRepository> ViewService for PostTermService<R> {
    type Entity = PostTerm;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

#[derive(Clone)]
pub struct PostTermInfoService<R: PostTermInfoRepository> {
    repository: Arc<R>,
}

impl<R: PostTermInfoRepository> PostTermInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: PostTermInfoRepository> ViewService for PostTermInfoService<R> {
    type Entity = PostTermInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
