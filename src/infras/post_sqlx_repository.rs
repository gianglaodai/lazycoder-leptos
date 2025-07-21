#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::post_service::{Post, PostRepository, PostStatus};
use crate::business::repository::{Repository, SortCriterion};
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxRepository;
use sqlx::PgPool;
use std::collections::HashMap;
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
    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<Post>, CoreError> {
        SqlxRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }

    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        SqlxRepository::count(self, filters).await
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
            CoreError::UnprocessableEntity("id_is_required".into(), HashMap::new())
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

    async fn find_by_author(&self, author_id: i32) -> Result<Vec<Post>, CoreError> {
        let result = sqlx::query_as::<_, PostOrm>("SELECT * FROM posts WHERE author_id=$1")
            .bind(author_id)
            .fetch_all(self.get_pool())
            .await?;

        Ok(result.into_iter().map(Self::from_orm).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::business::filter::{Filter, FilterOperator, FilterValue};
    use crate::business::repository::SortCriterion;
    use sqlx::{postgres::PgPoolOptions, PgPool};

    #[tokio::test]
    async fn test_build_find_many_query() {
        let pool: PgPool = PgPoolOptions::new()
            .connect_lazy("postgres://user:pass@localhost:5432/test")
            .unwrap();
        let repo = PostSqlxRepository::new(pool);
        let filters = vec![
            Filter::Property {
                property_name: "title".into(),
                operator: FilterOperator::Equal,
                value: FilterValue::String("test".into()),
            },
            Filter::Property {
                property_name: "slug".into(),
                operator: FilterOperator::Equal,
                value: FilterValue::String("test".into()),
            },
            Filter::Attribute {
                attr_name: "status".into(),
                operator: FilterOperator::Equal,
                value: FilterValue::Int(1),
            },
            Filter::Attribute {
                attr_name: "status".into(),
                operator: FilterOperator::In,
                value: FilterValue::ListInt(vec![2, 3]),
            },
        ];
        let sorts = vec![SortCriterion {
            field: "title".into(),
            ascending: true,
        }];
        let query = repo.build_find_many_query(sorts, None, None, filters, false);
        assert_eq!(query.sql(), "SELECT * FROM posts WHERE title = $1 AND slug = $2 AND EXISTS (SELECT 1 FROM attribute_values av JOIN attributes a ON a.id = av.attribute_id WHERE av.entity_id = posts.id AND av.entity_type = $3 AND a.name = $4 AND av.int_value = $5) AND EXISTS (SELECT 1 FROM attribute_values av JOIN attributes a ON a.id = av.attribute_id WHERE av.entity_id = posts.id AND av.entity_type = $6 AND a.name = $7 AND av.int_value IN  (($8), ($9)) ) ORDER BY title ASC OFFSET 0 LIMIT ALL");
    }
}
