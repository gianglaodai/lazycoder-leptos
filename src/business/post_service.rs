use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::Repository;
use crate::define_struct_with_common_fields;
use std::future::Future;
use std::sync::Arc;
use crate::business::sort::SortCriterion;

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

impl std::str::FromStr for PostStatus {
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
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: PostStatus,
    pub author_id: Option<i32>,
});

pub trait PostRepository: Repository<Post> + Send + Sync {
    fn find_by_slug(&self, slug: &str) -> impl Future<Output = Result<Option<Post>, CoreError>>;
    fn find_by_author(&self, author_id: i32) -> impl Future<Output = Result<Vec<Post>, CoreError>>;
}
#[derive(Clone)]
pub struct PostService<R: PostRepository> {
    post_repository: Arc<R>,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(post_repository: Arc<R>) -> Self {
        Self { post_repository }
    }

    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<Post>, CoreError> {
        self.post_repository.find_all(filters).await
    }
    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<Post>, CoreError> {
        self.post_repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.post_repository.count(filters).await
    }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<Post>, CoreError> {
        self.post_repository.find_by_id(id).await
    }
    pub async fn get_by_uid(&self, uid: String) -> Result<Option<Post>, CoreError> {
        self.post_repository.find_by_uid(uid).await
    }
    pub async fn create(&self, post: &Post) -> Result<Post, CoreError> {
        self.post_repository.create(post).await
    }
    pub async fn update(&self, post: &Post) -> Result<Post, CoreError> {
        self.post_repository.update(post).await
    }
    pub async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        self.post_repository.delete_by_id(id).await
    }
    pub async fn delete_by_uid(&self, uid: String) -> Result<u64, CoreError> {
        self.post_repository.delete_by_uid(uid).await
    }
    pub async fn get_by_slug(&self, slug: &str) -> Result<Option<Post>, CoreError> {
        self.post_repository.find_by_slug(slug).await
    }
}
