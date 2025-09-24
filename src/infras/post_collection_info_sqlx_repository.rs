#![cfg(feature = "ssr")]

use crate::business::post_collection_service::{PostCollectionInfo, PostCollectionInfoRepository};
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxViewMeta, SqlxViewRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostCollectionInfoSqlxRepository {
    pool: PgPool,
}

define_readonly_orm_with_common_fields!(PostCollectionInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub visibility: String,
});

impl PostCollectionInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["slug", "title", "description"]
    }
}

impl From<PostCollectionInfoOrm> for PostCollectionInfo {
    fn from(orm: PostCollectionInfoOrm) -> Self {
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

impl PostCollectionInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostCollectionInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_collections_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostCollectionInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostCollectionInfoOrm::searchable_columns()
    }
}

impl SqlxViewRepository for PostCollectionInfoSqlxRepository {
    type Entity = PostCollectionInfo;
    type Orm = PostCollectionInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostCollectionInfo::from(orm)
    }
}

impl PostCollectionInfoRepository for PostCollectionInfoSqlxRepository {}
