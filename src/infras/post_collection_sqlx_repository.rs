#![cfg(feature = "ssr")]

use std::collections::HashMap;
use std::future::Future;
use crate::business::post_collection_service::{
    PostCollection, PostCollectionCreate, PostCollectionRepository,
};
use crate::common::repository::{Repository, ViewRepository};
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxRepository, SqlxViewRepository};
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;
use crate::common::error::CoreError;
use crate::common::filter::{Filter, ScalarValue};
use crate::common::sort::SortCriterion;

#[derive(Clone)]
pub struct PostCollectionSqlxRepository {
    pool: PgPool,
}

// ORM for table post_collections
define_orm_with_common_fields!(PostCollection {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub visibility: String,
});

impl PostCollectionOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["slug", "title", "description", "visibility"]
    }
}

impl From<PostCollectionOrm> for PostCollection {
    fn from(orm: PostCollectionOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            slug: orm.slug,
            title: orm.title,
            description: orm.description,
            visibility: orm.visibility,
        }
    }
}

impl PostCollectionSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<PostCollection> for PostCollectionSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_collections"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostCollectionOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostCollectionOrm::searchable_columns()
    }

    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        SqlxViewRepository::count(self, filters).await
    }
    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostCollection>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }
    async fn find_by_id(&self, id: i32) -> Result<Option<PostCollection>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }
    async fn find_by_uid(&self, uid: String) -> Result<Option<PostCollection>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
    async fn get_column_type_map(
        &self,
    ) -> Result<HashMap<String, ScalarValue>, CoreError>
    {
        SqlxViewRepository::get_column_type_map(self).await
    }
}

impl SqlxViewRepository for PostCollectionSqlxRepository {
    type Entity = PostCollection;
    type Orm = PostCollectionOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostCollection::from(orm)
    }
}

impl SqlxRepository for PostCollectionSqlxRepository {
    type EntityCreate = PostCollectionCreate;

    async fn get_attribute_type_map(&self) -> Result<HashMap<String, ScalarValue>, CoreError> {
        SqlxRepository::get_attribute_type_map(self)
    }

    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_id(self, id)
    }

    async fn delete_by_ids(&self, ids: Vec<i32>) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_ids(self, ids)
    }

    async fn delete_by_uid(&self, uid: Uuid) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_uid(self, uid)
    }
}

impl Repository<PostCollection, PostCollectionCreate> for PostCollectionSqlxRepository {
    fn delete_by_id(&self, id: i32) -> impl Future<Output = Result<u64, CoreError>> + Send {
        SqlxRepository::delete_by_id(self, id)
    }
    fn delete_by_ids(&self, ids: Vec<i32>) -> impl Future<Output = Result<u64, CoreError>> + Send {
        SqlxRepository::delete_by_ids(self, ids)
    }
    fn delete_by_uid(&self, uid: String) -> impl Future<Output = Result<u64, CoreError>> + Send {
        SqlxRepository::delete_by_uid(self, Uuid::parse_str(&uid).unwrap())
    }
    fn delete_by_uids(&self, uids: Vec<String>) -> impl Future<Output=Result<u64, CoreError>> {
        SqlxRepository::delete_by_uids(self, uids.iter().map(Uuid::parse_str).collect())
    }

    fn create(&self, create: &PostCollectionCreate) -> impl Future<Output = Result<PostCollection, CoreError>> + Send {
        let pool = self.pool.clone();
        let create = create.clone();
        async move {
            let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
                "INSERT INTO post_collections (uid, version, slug, title, description, visibility) ",
            );
            qb.push("VALUES (")
                .push_bind(Uuid::new_v4())
                .push(", ")
                .push_bind(0i32)
                .push(", ")
                .push_bind(create.slug)
                .push(", ")
                .push_bind(create.title)
                .push(", ")
                .push_bind(create.description)
                .push(", ")
                .push_bind(create.visibility)
                .push(") RETURNING ")
                .push("id, uid, version, created_at, updated_at, slug, title, description, visibility");

            let row = qb.build().fetch_one(&pool).await?;
            let orm: PostCollectionOrm = sqlx::FromRow::from_row(&row)?;
            Ok(PostCollection::from(orm))
        }
    }

    fn update(&self, entity: &PostCollection) -> impl Future<Output = Result<PostCollection, CoreError>> + Send {
        use sqlx::QueryBuilder;
        use sqlx::Postgres;
        let pool = self.pool.clone();
        let e = entity.clone();
        async move {
            let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(
                "UPDATE post_collections SET version = ",
            );
            qb.push_bind(e.version + 1)
                .push(", slug = ")
                .push_bind(e.slug)
                .push(", title = ")
                .push_bind(e.title)
                .push(", description = ")
                .push_bind(e.description)
                .push(", visibility = ")
                .push_bind(e.visibility)
                .push(" WHERE id = ")
                .push_bind(e.id)
                .push(" RETURNING ")
                .push("id, uid, version, created_at, updated_at, slug, title, description, visibility");

            let row = qb.build().fetch_one(&pool).await?;
            let orm: PostCollectionOrm = sqlx::FromRow::from_row(&row)?;
            Ok(PostCollection::from(orm))
        }
    }

    fn get_attribute_type_map(&self) -> impl Future<Output = Result<HashMap<String, ScalarValue>, CoreError>> + Send {
        SqlxRepository::get_attribute_type_map(self)
    }
}

impl PostCollectionRepository for PostCollectionSqlxRepository {}
