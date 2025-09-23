#![cfg(feature = "ssr")]

use crate::business::post_service::{Post, PostCreate, PostRepository, PostStatus};
use crate::common::repository::{Repository, ViewRepository};
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxRepository, SqlxViewRepository};
use sqlx::PgPool;
use std::collections::HashMap;
use std::future::Future;
use uuid::Uuid;
use crate::common::error::CoreError;
use crate::common::filter::{Filter, ScalarValue};
use crate::common::sort::SortCriterion;

#[derive(Clone)]
pub struct PostSqlxRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(Post {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: i32,
    pub user_id: i32,
    pub type_id: i32,
});

impl PostOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["slug", "title", "summary", "content"]
    }
}

impl From<PostOrm> for Post {
    fn from(orm: PostOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            slug: orm.slug,
            title: orm.title,
            summary: orm.summary,
            content: orm.content,
            status: PostStatus::from(orm.status),
            user_id: orm.user_id,
            type_id: orm.type_id,
        }
    }
}

impl PostSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<Post> for PostSqlxRepository {
    fn get_table_name(&self) -> &str {
        "posts"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostOrm::searchable_columns()
    }
    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        SqlxViewRepository::count(self, filters).await
    }

    async fn get_column_type_map(&self) -> Result<HashMap<String, ScalarValue>, CoreError> {
        SqlxViewRepository::get_column_type_map(self).await
    }

    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<Post>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Post>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }

    async fn find_by_uid(&self, uid: String) -> Result<Option<Post>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
}

impl Repository<Post, PostCreate> for PostSqlxRepository {
    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_id(self, id).await
    }
    async fn delete_by_ids(&self, ids: Vec<i32>) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_ids(self, ids).await
    }

    async fn delete_by_uid(&self, uid: String) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
    async fn delete_by_uids(&self, uids: Vec<String>) -> impl Future<Output=Result<u64, CoreError>> {
        SqlxRepository::delete_by_uids(self, uids.iter().map(Uuid::parse_str).collect())
    }

    async fn create(&self, post_create: &PostCreate) -> Result<Post, CoreError> {
        let now = time::OffsetDateTime::now_utc();
        let row: PostOrm = sqlx::query_as::<_, PostOrm>(
            "INSERT INTO posts (uid, created_at, updated_at, slug, title, summary, content, status, user_id, type_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) returning *",
        )
            .bind(Uuid::now_v7())
            .bind(&now)
            .bind(&now)
            .bind(&post_create.slug)
            .bind(&post_create.title)
            .bind(&post_create.summary)
            .bind(&post_create.content)
            .bind(&post_create.status.as_i32())
            .bind(&post_create.user_id)
            .bind(&post_create.type_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(Post::from(row))
    }

    async fn update(&self, post: &Post) -> Result<Post, CoreError> {
        let now = time::OffsetDateTime::now_utc();
        let post = sqlx::query_as::<_, PostOrm>(
            "UPDATE posts SET slug=$1, title=$2, summary=$3, content=$4, status=$5, type_id=$6, updated_at=$7 WHERE id=$8 RETURNING *",
        )
            .bind(&post.slug)
            .bind(&post.title)
            .bind(&post.summary)
            .bind(&post.content)
            .bind(post.status.as_i32())
            .bind(post.type_id)
            .bind(now)
            .bind(post.id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Post::from(post))
    }

    fn get_attribute_type_map(
        &self,
    ) -> impl Future<Output = Result<HashMap<String, ScalarValue>, CoreError>> {
        SqlxRepository::get_attribute_type_map(self)
    }
}

impl SqlxViewRepository for PostSqlxRepository {
    type Entity = Post;
    type Orm = PostOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        Post::from(orm)
    }
}
impl SqlxRepository for PostSqlxRepository {
    type EntityCreate = PostCreate;
}

impl PostRepository for PostSqlxRepository {
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Post>, CoreError> {
        let result = sqlx::query_as::<_, PostOrm>("SELECT * FROM posts WHERE slug=$1")
            .bind(&slug)
            .fetch_optional(self.get_pool())
            .await?;

        Ok(result.map(Self::from_orm))
    }

    async fn find_by_author(&self, user_id: i32) -> Result<Vec<Post>, CoreError> {
        let result = sqlx::query_as::<_, PostOrm>("SELECT * FROM posts WHERE user_id=$1")
            .bind(user_id)
            .fetch_all(self.get_pool())
            .await?;

        Ok(result.into_iter().map(Self::from_orm).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use crate::common::filter::{FilterOperator, FilterValue};

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
                value: FilterValue::Single(ScalarValue::String("test".into())),
            },
            Filter::Property {
                property_name: "slug".into(),
                operator: FilterOperator::Equal,
                value: FilterValue::Single(ScalarValue::String("test".into())),
            },
            Filter::Attribute {
                attr_name: "status".into(),
                operator: FilterOperator::Equal,
                value: FilterValue::Single(ScalarValue::Int(1)),
            },
            Filter::Attribute {
                attr_name: "status".into(),
                operator: FilterOperator::In,
                value: FilterValue::List(vec![ScalarValue::Int(2), ScalarValue::Int(3)]),
            },
            Filter::Search {
                value: "abc xyz".to_owned(),
            },
        ];
        let sorts = vec![SortCriterion {
            field: "title".into(),
            ascending: true,
        }];
        let query = repo.build_find_many_query(sorts, None, None, filters, false);
        assert_eq!(query.sql(), "SELECT * FROM posts WHERE title = $1 AND slug = $2 AND EXISTS (SELECT 1 FROM attribute_values av JOIN attributes a ON a.id = av.attribute_id WHERE av.entity_id = posts.id AND av.entity_type = $3 AND a.name = $4 AND av.int_value = $5) AND EXISTS (SELECT 1 FROM attribute_values av JOIN attributes a ON a.id = av.attribute_id WHERE av.entity_id = posts.id AND av.entity_type = $6 AND a.name = $7 AND av.int_value IN  (($8), ($9)) ) AND ((to_tsvector('simple', unaccent(coalesce(slug, '') || ' ' || coalesce(title, '') || ' ' || coalesce(summary, '') || ' ' || coalesce(content, ''))) @@ plainto_tsquery('simple', unaccent($10))) OR  (EXISTS ( SELECT 1 FROM attribute_values av JOIN attributes a ON a.id = av.attribute_id WHERE av.entity_id = posts.id AND av.entity_type = $11 AND to_tsvector('simple', unaccent(coalesce(av.string_value, ''))) @@ plainto_tsquery('simple', unaccent($12))))) ORDER BY title ASC OFFSET 0 LIMIT ALL");
    }
}
