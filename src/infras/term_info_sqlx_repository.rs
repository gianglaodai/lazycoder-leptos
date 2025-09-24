#![cfg(feature = "ssr")]

use crate::business::term_service::{TermInfo, TermInfoRepository};
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxViewMeta, SqlxViewRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct TermInfoSqlxRepository {
    pool: PgPool,
}

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
