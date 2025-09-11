#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::post_type_service::{PostType, PostTypeCreate, PostTypeRepository};
use crate::business::repository::{Repository, ViewRepository};
use crate::business::sort::SortCriterion;
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxRepository, SqlxViewRepository};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostTypeSqlxRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(PostType {
    pub code: String,
    pub name: String,
});

impl PostTypeOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["code", "name"]
    }
}

impl From<PostTypeOrm> for PostType {
    fn from(orm: PostTypeOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            code: orm.code,
            name: orm.name,
        }
    }
}

impl PostTypeSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewRepository<PostType> for PostTypeSqlxRepository {
    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        SqlxViewRepository::count(self, filters).await
    }

    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<PostType>, CoreError> {
        SqlxViewRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<PostType>, CoreError> {
        SqlxViewRepository::find_by_id(self, id).await
    }

    async fn find_by_uid(&self, uid: String) -> Result<Option<PostType>, CoreError> {
        SqlxViewRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }
}

impl Repository<PostType, PostTypeCreate> for PostTypeSqlxRepository {
    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_id(self, id).await
    }
    async fn delete_by_ids(&self, ids: Vec<i32>) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_ids(self, ids).await
    }

    async fn delete_by_uid(&self, uid: String) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }

    async fn create(&self, create: &PostTypeCreate) -> Result<PostType, CoreError> {
        let now = time::OffsetDateTime::now_utc();
        let row: PostTypeOrm = sqlx::query_as::<_, PostTypeOrm>(
            "INSERT INTO post_types (uid, created_at, updated_at, code, name) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(Uuid::now_v7())
        .bind(&now)
        .bind(&now)
        .bind(&create.code)
        .bind(&create.name)
        .fetch_one(&self.pool)
        .await?;
        Ok(PostType::from(row))
    }

    async fn update(&self, entity: &PostType) -> Result<PostType, CoreError> {
        let now = time::OffsetDateTime::now_utc();
        let row = sqlx::query_as::<_, PostTypeOrm>(
            "UPDATE post_types SET code=$1, name=$2, updated_at=$3 WHERE id=$4 RETURNING *",
        )
        .bind(&entity.code)
        .bind(&entity.name)
        .bind(now)
        .bind(entity.id)
        .fetch_one(&self.pool)
        .await?;
        Ok(PostType::from(row))
    }
}

impl SqlxViewRepository for PostTypeSqlxRepository {
    type Entity = PostType;
    type Orm = PostTypeOrm;
    fn get_table_name(&self) -> &str {
        "post_types"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostTypeOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostTypeOrm::searchable_columns()
    }
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostType::from(orm)
    }
}

impl SqlxRepository for PostTypeSqlxRepository {
    type EntityCreate = PostTypeCreate;
}

impl PostTypeRepository for PostTypeSqlxRepository {}
