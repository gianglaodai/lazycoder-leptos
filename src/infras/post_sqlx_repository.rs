#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use crate::business::post_service::{Post, PostRepository, PostStatus};
use crate::business::repository::{Repository, SortCriterion};
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostSqlxRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(PostOrm {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: i32,
    pub author_id: Option<i32>,
});

impl From<PostOrm> for Post {
    fn from(post: PostOrm) -> Self {
        Self {
            id: post.id,
            uid: post.uid,
            created_at: post.created_at,
            updated_at: post.updated_at,
            slug: post.slug,
            title: post.title,
            summary: post.summary,
            content: post.content,
            status: PostStatus::from(post.status),
            author_id: post.author_id,
        }
    }
}

impl PostSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Repository<Post> for PostSqlxRepository {
    async fn find_all(&self) -> Result<Vec<Post>, CoreError> {
        SqlxRepository::find_all(self).await
    }

    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
    ) -> Result<Vec<Post>, CoreError> {
        SqlxRepository::find_many(self, sort_criteria, first_result, max_results).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Post>, CoreError> {
        SqlxRepository::find_by_id(self, id).await
    }

    async fn find_by_uid(&self, uid: Uuid) -> Result<Option<Post>, CoreError> {
        SqlxRepository::find_by_uid(self, uid).await
    }

    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_id(self, id).await
    }

    async fn delete_by_uid(&self, uid: Uuid) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_uid(self, uid).await
    }
    async fn create(&self, post: &Post) -> Result<Post, CoreError> {
        let now = time::OffsetDateTime::now_utc();
        let row: PostOrm = sqlx::query_as::<_, PostOrm>(
            "INSERT INTO posts (uid, created_at, updated_at, slug, title, summary, content, status) values ($1, $2, $3, $4, $5, $6, $7, $8) returning *",
        )
            .bind(Uuid::now_v7())
            .bind(&now)
            .bind(&now)
            .bind(&post.slug)
            .bind(&post.title)
            .bind(&post.summary)
            .bind(&post.content)
            .bind(&post.status.as_i32())
            .fetch_one(&self.pool)
            .await?;
        Ok(Post::from(row))
    }

    async fn update(&self, post: &Post) -> Result<Post, CoreError> {
        let id = post.id.ok_or_else(|| {
            CoreError::ValidationError("Post ID is required for update".to_string())
        })?;
        let now = time::OffsetDateTime::now_utc();
        let post = sqlx::query_as::<_, PostOrm>(
            "UPDATE posts SET slug=$1, title=$2, summary=$3, content=$4, status=$5, updated_at=$6 WHERE id=$7 RETURNING *",
        )
            .bind(&post.slug)
            .bind(&post.title)
            .bind(&post.summary)
            .bind(&post.content)
            .bind(post.status.as_i32())
            .bind(now)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Post::from(post))
    }
}

impl SqlxRepository for PostSqlxRepository {
    type Entity = Post;
    type Orm = PostOrm;

    fn get_table_name(&self) -> &str {
        "posts"
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    fn from_orm(orm: Self::Orm) -> Self::Entity {
        Post::from(orm)
    }
}

impl PostRepository for PostSqlxRepository {
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Post>, CoreError> {
        let result = sqlx::query_as::<_, PostOrm>("SELECT * FROM posts WHERE slug=$1")
            .bind(&slug)
            .fetch_optional(self.get_pool())
            .await?;

        Ok(result.map(Self::from_orm))
    }
}
