#![cfg(feature = "ssr")]

use crate::business::post_collection_item_service::{
    PostCollectionItemInfo, PostCollectionItemInfoRepository,
};
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxViewMeta, SqlxViewRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostCollectionItemInfoSqlxRepository {
    pool: PgPool,
}

define_readonly_orm_with_common_fields!(PostCollectionItemInfo {
    pub post_collection_id: i32,
    pub post_id: i32,
    pub position: i32,
    pub headline: Option<String>,
    pub collection_slug: Option<String>,
    pub collection_title: Option<String>,
    pub post_slug: Option<String>,
    pub post_title: Option<String>,
});

impl PostCollectionItemInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec![
            "collection_slug",
            "collection_title",
            "post_slug",
            "post_title",
        ]
    }
}

impl From<PostCollectionItemInfoOrm> for PostCollectionItemInfo {
    fn from(orm: PostCollectionItemInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            post_collection_id: orm.post_collection_id,
            post_id: orm.post_id,
            position: orm.position,
            headline: orm.headline,
            collection_slug: orm.collection_slug,
            collection_title: orm.collection_title,
            post_slug: orm.post_slug,
            post_title: orm.post_title,
        }
    }
}

impl PostCollectionItemInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostCollectionItemInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_collection_items_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostCollectionItemInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostCollectionItemInfoOrm::searchable_columns()
    }
}

impl SqlxViewRepository for PostCollectionItemInfoSqlxRepository {
    type Entity = PostCollectionItemInfo;
    type Orm = PostCollectionItemInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostCollectionItemInfo::from(orm)
    }
}

impl PostCollectionItemInfoRepository for PostCollectionItemInfoSqlxRepository {}
