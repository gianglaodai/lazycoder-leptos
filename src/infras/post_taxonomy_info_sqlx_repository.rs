#![cfg(feature = "ssr")]

use crate::business::post_taxonomy_service::{PostTaxonomyInfo, PostTaxonomyInfoRepository};
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxViewMeta, SqlxViewRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostTaxonomyInfoSqlxRepository {
    pool: PgPool,
}

define_readonly_orm_with_common_fields!(PostTaxonomyInfo { pub code: String, pub name: String, });

impl PostTaxonomyInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["code", "name"]
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
