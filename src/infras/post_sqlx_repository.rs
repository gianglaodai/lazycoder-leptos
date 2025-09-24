#![cfg(feature = "ssr")]

use crate::business::post_service::{Post, PostCreate, PostRepository, PostStatus};
use crate::common::error::CoreError;
use crate::common::filter::{Filter, ScalarValue};
use crate::common::repository::{Repository, ViewRepository};
use crate::common::sort::SortCriterion;
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::{
    SqlxEntityMapper, SqlxRepository, SqlxViewMeta, SqlxViewRepository,
};
use sqlx::PgPool;
use std::future::Future;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostSqlxRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(Post {
    pub title: String,
    pub type_id: i32,
    pub slug: String,
    pub summary: String,
    pub content: String,
    pub status: i32,
    pub user_id: i32,
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

impl SqlxViewMeta for PostSqlxRepository {
    fn get_table_name(&self) -> &str {
        "posts"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostOrm::searchable_columns()
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
impl SqlxEntityMapper for PostSqlxRepository {
    type Entity = Post;
    type EntityCreate = PostCreate;
    type Orm = PostOrm;

    fn to_orm_from_create(&self, create: &Self::EntityCreate) -> Self::Orm {
        let now = time::OffsetDateTime::now_utc();
        let slug = create.title.to_lowercase().replace(' ', "-");
        PostOrm {
            id: 0,
            uid: Uuid::now_v7(),
            version: 0,
            created_at: now,
            updated_at: now,
            slug,
            title: create.title.clone(),
            summary: String::new(),
            content: String::new(),
            status: PostStatus::DRAFT.as_i32(),
            user_id: create.user_id,
            type_id: create.type_id,
        }
    }

    fn to_orm_from_entity(&self, entity: &Self::Entity) -> Self::Orm {
        PostOrm {
            id: entity.id,
            uid: Uuid::parse_str(&entity.uid).unwrap_or_else(|_| Uuid::nil()),
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            slug: entity.slug.clone(),
            title: entity.title.clone(),
            summary: entity.summary.clone(),
            content: entity.content.clone(),
            status: entity.status.as_i32(),
            user_id: entity.user_id,
            type_id: entity.type_id,
        }
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
    use crate::common::filter::{FilterOperator, FilterValue};
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
