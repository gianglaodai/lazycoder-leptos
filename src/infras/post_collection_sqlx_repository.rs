#![cfg(feature = "ssr")]

use crate::business::post_collection_service::{
    PostCollection, PostCollectionCreate, PostCollectionInfo, PostCollectionInfoRepository,
    PostCollectionRepository,
};
use crate::infras::sqlx_repository::{
    SqlxEntityMapper, SqlxRepository, SqlxViewMeta, SqlxViewRepository,
};
use crate::{define_orm_with_common_fields, define_readonly_orm_with_common_fields};
use sqlx::PgPool;
use uuid::Uuid;

define_orm_with_common_fields!(PostCollection {
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub visibility: String,
});

define_readonly_orm_with_common_fields!(PostCollectionInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub visibility: String,
});
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

#[derive(Clone)]
pub struct PostCollectionSqlxRepository {
    pool: PgPool,
}
#[derive(Clone)]
pub struct PostCollectionInfoSqlxRepository {
    pool: PgPool,
}
impl PostCollectionOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["slug", "title", "description", "visibility"]
    }
}

impl PostCollectionSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostCollectionSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_collections"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostCollectionOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostCollectionOrm::searchable_columns()
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

impl SqlxEntityMapper for PostCollectionSqlxRepository {
    type Entity = PostCollection;
    type EntityCreate = PostCollectionCreate;
    type Orm = PostCollectionOrm;

    fn to_orm_from_create(&self, create: &Self::EntityCreate) -> Self::Orm {
        let now = time::OffsetDateTime::now_utc();
        PostCollectionOrm {
            id: 0,
            uid: Uuid::now_v7(),
            version: 0,
            created_at: now,
            updated_at: now,
            slug: create.slug.clone(),
            title: create.title.clone(),
            description: None,
            visibility: create.visibility.clone(),
        }
    }

    fn to_orm_from_entity(&self, entity: &Self::Entity) -> Self::Orm {
        PostCollectionOrm {
            id: entity.id,
            uid: Uuid::parse_str(&entity.uid).unwrap_or_else(|_| Uuid::nil()),
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            slug: entity.slug.clone(),
            title: entity.title.clone(),
            description: entity.description.clone(),
            visibility: entity.visibility.clone(),
        }
    }
}

impl SqlxRepository for PostCollectionSqlxRepository {
    type EntityCreate = PostCollectionCreate;
}

impl PostCollectionRepository for PostCollectionSqlxRepository {}

impl PostCollectionInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["slug", "title", "description", "visibility"]
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
