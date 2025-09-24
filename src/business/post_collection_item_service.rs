use crate::common::repository::ViewRepository;
use crate::common::service::ViewService;
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::sync::Arc;

define_struct_with_common_fields!(PostCollectionItem {
    req {
        pub post_collection_id: i32,
        pub post_id: i32,
        pub position: i32,
    }
    opt {
        pub headline: Option<String>,
    }
});

define_readonly_struct_with_common_fields!(PostCollectionItemInfo {
    pub post_collection_id: i32,
    pub post_id: i32,
    pub position: i32,
    pub headline: Option<String>,
    pub collection_slug: Option<String>,
    pub collection_title: Option<String>,
    pub post_slug: Option<String>,
    pub post_title: Option<String>,
});

pub trait PostCollectionItemRepository: ViewRepository<PostCollectionItem> + Send + Sync {}
pub trait PostCollectionItemInfoRepository:
    ViewRepository<PostCollectionItemInfo> + Send + Sync
{
}

#[derive(Clone)]
pub struct PostCollectionItemService<R: PostCollectionItemRepository> {
    repository: Arc<R>,
}

impl<R: PostCollectionItemRepository> PostCollectionItemService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: PostCollectionItemRepository> ViewService for PostCollectionItemService<R> {
    type Entity = PostCollectionItem;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

#[derive(Clone)]
pub struct PostCollectionItemInfoService<R: PostCollectionItemInfoRepository> {
    repository: Arc<R>,
}

impl<R: PostCollectionItemInfoRepository> PostCollectionItemInfoService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

impl<R: PostCollectionItemInfoRepository> ViewService for PostCollectionItemInfoService<R> {
    type Entity = PostCollectionItemInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}
