#![cfg(feature = "ssr")]

use crate::business::post_taxonomy_service::{
    PostTaxonomy, PostTaxonomyCreate, PostTaxonomyInfo, PostTaxonomyInfoRepository,
    PostTaxonomyRepository,
};
use crate::infras::sqlx_repository::{
    SqlxEntityMapper, SqlxRepository, SqlxViewMeta, SqlxViewRepository,
};
use crate::{define_orm_with_common_fields, define_readonly_orm_with_common_fields};
use sqlx::PgPool;
use uuid::Uuid;

// Table
define_orm_with_common_fields!(PostTaxonomy { pub code: String, pub name: String, });

// View
define_readonly_orm_with_common_fields!(PostTaxonomyInfo { pub code: String, pub name: String, });

impl From<PostTaxonomyOrm> for PostTaxonomy {
    fn from(orm: PostTaxonomyOrm) -> Self {
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
impl From<PostTaxonomyInfoOrm> for PostTaxonomyInfo {
    fn from(orm: PostTaxonomyInfoOrm) -> Self {
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
#[derive(Clone)]
pub struct PostTaxonomySqlxRepository {
    pool: PgPool,
}
#[derive(Clone)]
pub struct PostTaxonomyInfoSqlxRepository {
    pool: PgPool,
}

impl PostTaxonomyOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["code", "name"]
    }
}

impl PostTaxonomySqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostTaxonomySqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_taxonomies"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostTaxonomyOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostTaxonomyOrm::searchable_columns()
    }
}

impl SqlxViewRepository for PostTaxonomySqlxRepository {
    type Entity = PostTaxonomy;
    type Orm = PostTaxonomyOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostTaxonomy::from(orm)
    }
}

impl SqlxEntityMapper for PostTaxonomySqlxRepository {
    type Entity = PostTaxonomy;
    type EntityCreate = PostTaxonomyCreate;
    type Orm = PostTaxonomyOrm;

    fn to_orm_from_create(&self, create: &Self::EntityCreate) -> Self::Orm {
        let now = time::OffsetDateTime::now_utc();
        PostTaxonomyOrm {
            id: 0,
            uid: Uuid::now_v7(),
            version: 0,
            created_at: now,
            updated_at: now,
            code: create.code.clone(),
            name: create.name.clone(),
        }
    }

    fn to_orm_from_entity(&self, entity: &Self::Entity) -> Self::Orm {
        PostTaxonomyOrm {
            id: entity.id,
            uid: Uuid::parse_str(&entity.uid).unwrap_or_else(|_| Uuid::nil()),
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            code: entity.code.clone(),
            name: entity.name.clone(),
        }
    }
}

impl SqlxRepository for PostTaxonomySqlxRepository {
    type EntityCreate = PostTaxonomyCreate;
}

impl PostTaxonomyRepository for PostTaxonomySqlxRepository {}

impl PostTaxonomyInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["code", "name"]
    }
}

impl PostTaxonomyInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostTaxonomyInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_taxonomies_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostTaxonomyInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostTaxonomyInfoOrm::searchable_columns()
    }
}

impl SqlxViewRepository for PostTaxonomyInfoSqlxRepository {
    type Entity = PostTaxonomyInfo;
    type Orm = PostTaxonomyInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostTaxonomyInfo::from(orm)
    }
}

impl PostTaxonomyInfoRepository for PostTaxonomyInfoSqlxRepository {}
