#![cfg(feature = "ssr")]

use crate::business::post_type_service::{PostTypeInfo, PostTypeInfoRepository};
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxViewMeta, SqlxViewRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostTypeInfoSqlxRepository {
    pool: PgPool,
}

define_readonly_orm_with_common_fields!(PostTypeInfo {
    pub code: String,
    pub name: String,
});

impl PostTypeInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["code", "name"]
    }
}

impl From<PostTypeInfoOrm> for PostTypeInfo {
    fn from(orm: PostTypeInfoOrm) -> Self {
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

impl PostTypeInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostTypeInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_types_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostTypeInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostTypeInfoOrm::searchable_columns()
    }
}

impl SqlxViewRepository for PostTypeInfoSqlxRepository {
    type Entity = PostTypeInfo;
    type Orm = PostTypeInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostTypeInfo::from(orm)
    }
}

impl PostTypeInfoRepository for PostTypeInfoSqlxRepository {}
