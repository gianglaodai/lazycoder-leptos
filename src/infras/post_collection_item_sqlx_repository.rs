#![cfg(feature = "ssr")]

use crate::business::post_collection_item_service::{
    PostCollectionItem, PostCollectionItemCreate, PostCollectionItemInfo,
    PostCollectionItemInfoRepository, PostCollectionItemRepository,
};
use crate::infras::sqlx_repository::{
    SqlxEntityMapper, SqlxRepository, SqlxViewMeta, SqlxViewRepository,
};
use crate::{define_orm_with_common_fields, define_readonly_orm_with_common_fields};
use sqlx::PgPool;

define_orm_with_common_fields!(PostCollectionItem {
    pub post_collection_id: i32,
    pub post_id: i32,
    pub position: i32,
    pub headline: Option<String>,
});

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

impl From<PostCollectionItemOrm> for PostCollectionItem {
    fn from(orm: PostCollectionItemOrm) -> Self {
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
        }
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
#[derive(Clone)]
pub struct PostCollectionItemSqlxRepository {
    pool: PgPool,
}
#[derive(Clone)]
pub struct PostCollectionItemInfoSqlxRepository {
    pool: PgPool,
}

impl PostCollectionItemOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["headline"]
    }
}
impl PostCollectionItemSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostCollectionItemSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_collection_items"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostCollectionItemOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostCollectionItemOrm::searchable_columns()
    }
}

impl SqlxViewRepository for PostCollectionItemSqlxRepository {
    type Entity = PostCollectionItem;
    type Orm = PostCollectionItemOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostCollectionItem::from(orm)
    }
}

impl SqlxEntityMapper for PostCollectionItemSqlxRepository {
    type Entity = PostCollectionItem;
    type EntityCreate = PostCollectionItemCreate;
    type Orm = PostCollectionItemOrm;

    fn to_orm_from_create(&self, create: &Self::EntityCreate) -> Self::Orm {
        let now = time::OffsetDateTime::now_utc();
        PostCollectionItemOrm {
            id: 0,
            uid: uuid::Uuid::now_v7(),
            version: 0,
            created_at: now,
            updated_at: now,
            post_collection_id: create.post_collection_id,
            post_id: create.post_id,
            position: create.position,
            headline: None,
        }
    }

    fn to_orm_from_entity(&self, entity: &Self::Entity) -> Self::Orm {
        PostCollectionItemOrm {
            id: entity.id,
            uid: uuid::Uuid::parse_str(&entity.uid).unwrap_or_else(|_| uuid::Uuid::nil()),
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            post_collection_id: entity.post_collection_id,
            post_id: entity.post_id,
            position: entity.position,
            headline: entity.headline.clone(),
        }
    }
}

impl SqlxRepository for PostCollectionItemSqlxRepository {
    type EntityCreate = PostCollectionItemCreate;
}

impl PostCollectionItemRepository for PostCollectionItemSqlxRepository {}

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
