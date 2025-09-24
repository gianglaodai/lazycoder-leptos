use crate::common::error::CoreError;
use crate::common::repository::{Repository, ViewRepository};
use crate::common::service::{Service, ViewService};
use crate::{define_readonly_struct_with_common_fields, define_struct_with_common_fields};
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum PostStatus {
    DRAFT = 0,
    REVIEW = 1,
    PUBLISHED = 2,
    ARCHIVED = 3,
    DELETED = 4,
}

impl From<i32> for PostStatus {
    fn from(status: i32) -> Self {
        match status {
            0 => PostStatus::DRAFT,
            1 => PostStatus::REVIEW,
            2 => PostStatus::PUBLISHED,
            3 => PostStatus::ARCHIVED,
            4 => PostStatus::DELETED,
            _ => PostStatus::DRAFT,
        }
    }
}

impl PostStatus {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PostStatus::DRAFT => "DRAFT",
            PostStatus::REVIEW => "REVIEW",
            PostStatus::PUBLISHED => "PUBLISHED",
            PostStatus::ARCHIVED => "ARCHIVED",
            PostStatus::DELETED => "DELETED",
        }
    }
}

impl FromStr for PostStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DRAFT" => Ok(PostStatus::DRAFT),
            "REVIEW" => Ok(PostStatus::REVIEW),
            "PUBLISHED" => Ok(PostStatus::PUBLISHED),
            "ARCHIVED" => Ok(PostStatus::ARCHIVED),
            "DELETED" => Ok(PostStatus::DELETED),
            _ => Err(()),
        }
    }
}

define_struct_with_common_fields!(Post {
    req {
        pub title: String,
        pub type_id: i32,
    }
    opt {
        pub slug: String,
        pub summary: String,
        pub content: String,
        pub status: PostStatus,
        pub user_id: i32,
    }
});

define_readonly_struct_with_common_fields!(PostInfo {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: PostStatus,
    pub user_id: i32,
    pub username: String,
    pub email: String,
});

pub trait PostRepository: Repository<Post, PostCreate> + Send + Sync {
    fn find_by_slug(&self, slug: &str) -> impl Future<Output = Result<Option<Post>, CoreError>>;
    fn find_by_author(&self, user_id: i32) -> impl Future<Output = Result<Vec<Post>, CoreError>>;
}
#[derive(Clone)]
pub struct PostService<R: PostRepository> {
    repository: Arc<R>,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Option<Post>, CoreError> {
        self.repository.find_by_slug(slug).await
    }
}

impl<R: PostRepository> ViewService for PostService<R> {
    type Entity = Post;
    type Repo = R;

    fn get_repository(&self) -> &Self::Repo {
        &self.repository
    }
}

impl<R: PostRepository> Service for PostService<R> {
    type Create = PostCreate;
}

pub trait PostInfoRepository: ViewRepository<PostInfo> + Send + Sync {}

#[derive(Clone)]
pub struct PostInfoService<R: PostInfoRepository> {
    post_info_repository: Arc<R>,
}

impl<R: PostInfoRepository> PostInfoService<R> {
    pub fn new(post_info_repository: Arc<R>) -> Self {
        Self {
            post_info_repository,
        }
    }
}

impl<R: PostInfoRepository> ViewService for PostInfoService<R> {
    type Entity = PostInfo;
    type Repo = R;
    fn get_repository(&self) -> &Self::Repo {
        &self.post_info_repository
    }
}
