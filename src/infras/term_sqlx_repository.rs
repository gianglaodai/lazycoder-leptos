#![cfg(feature = "ssr")]

use crate::business::term_service::{
    Term, TermCreate, TermInfo, TermInfoRepository, TermRepository,
};
use crate::infras::sqlx_repository::{
    SqlxEntityMapper, SqlxRepository, SqlxViewMeta, SqlxViewRepository,
};
use crate::{define_orm_with_common_fields, define_readonly_orm_with_common_fields};
use sqlx::PgPool;
use uuid::Uuid;

// Table: terms
define_orm_with_common_fields!(Term {
    pub taxonomy_id: i32,
    pub slug: String,
    pub name: String,
    pub parent_id: Option<i32>,
    pub description: Option<String>,
});

// View: terms_info
define_readonly_orm_with_common_fields!(TermInfo {
    pub taxonomy_id: i32,
    pub taxonomy_code: String,
    pub taxonomy_name: String,
    pub parent_id: Option<i32>,
    pub parent_slug: Option<String>,
    pub parent_name: Option<String>,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
});

impl TermOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["slug", "name"]
    }
}

impl From<TermOrm> for Term {
    fn from(orm: TermOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            taxonomy_id: orm.taxonomy_id,
            slug: orm.slug,
            name: orm.name,
            parent_id: orm.parent_id,
            description: orm.description,
        }
    }
}

#[derive(Clone)]
pub struct TermSqlxRepository {
    pool: PgPool,
}
#[derive(Clone)]
pub struct TermInfoSqlxRepository {
    pool: PgPool,
}

impl TermSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for TermSqlxRepository {
    fn get_table_name(&self) -> &str {
        "terms"
    }
    fn get_columns(&self) -> Vec<&str> {
        TermOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        TermOrm::searchable_columns()
    }
}

impl SqlxViewRepository for TermSqlxRepository {
    type Entity = Term;
    type Orm = TermOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        Term::from(orm)
    }
}

impl SqlxEntityMapper for TermSqlxRepository {
    type Entity = Term;
    type EntityCreate = TermCreate;
    type Orm = TermOrm;

    fn to_orm_from_create(&self, create: &Self::EntityCreate) -> Self::Orm {
        let now = time::OffsetDateTime::now_utc();
        TermOrm {
            id: 0,
            uid: Uuid::now_v7(),
            version: 0,
            created_at: now,
            updated_at: now,
            taxonomy_id: create.taxonomy_id,
            slug: create.slug.clone(),
            name: create.name.clone(),
            parent_id: None,
            description: None,
        }
    }

    fn to_orm_from_entity(&self, entity: &Self::Entity) -> Self::Orm {
        TermOrm {
            id: entity.id,
            uid: Uuid::parse_str(&entity.uid).unwrap_or_else(|_| Uuid::nil()),
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            taxonomy_id: entity.taxonomy_id,
            slug: entity.slug.clone(),
            name: entity.name.clone(),
            parent_id: entity.parent_id,
            description: entity.description.clone(),
        }
    }
}

impl SqlxRepository for TermSqlxRepository {
    type EntityCreate = TermCreate;
}
impl TermRepository for TermSqlxRepository {}

impl TermInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["slug", "name", "taxonomy_code", "taxonomy_name"]
    }
}

impl From<TermInfoOrm> for TermInfo {
    fn from(orm: TermInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            taxonomy_id: orm.taxonomy_id,
            taxonomy_code: orm.taxonomy_code,
            taxonomy_name: orm.taxonomy_name,
            parent_id: orm.parent_id,
            parent_slug: orm.parent_slug,
            parent_name: orm.parent_name,
            slug: orm.slug,
            name: orm.name,
            description: orm.description,
        }
    }
}

impl TermInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for TermInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "terms_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        TermInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        TermInfoOrm::searchable_columns()
    }
}

impl SqlxViewRepository for TermInfoSqlxRepository {
    type Entity = TermInfo;
    type Orm = TermInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        TermInfo::from(orm)
    }
}

impl TermInfoRepository for TermInfoSqlxRepository {}
