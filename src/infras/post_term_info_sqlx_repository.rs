#![cfg(feature = "ssr")]
use crate::business::post_term_service::{PostTermInfo, PostTermInfoRepository};
use crate::define_readonly_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxViewMeta, SqlxViewRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostTermInfoSqlxRepository {
    pool: PgPool,
}

define_readonly_orm_with_common_fields!(PostTermInfo {
    pub post_id: i32,
    pub term_id: i32,
    pub post_slug: Option<String>,
    pub post_title: Option<String>,
    pub term_slug: Option<String>,
    pub term_name: Option<String>,
    pub taxonomy_id: Option<i32>,
    pub taxonomy_code: Option<String>,
});

impl PostTermInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec![
            "post_slug",
            "post_title",
            "term_slug",
            "term_name",
            "taxonomy_code",
        ]
    }
}

impl From<PostTermInfoOrm> for PostTermInfo {
    fn from(orm: PostTermInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            post_id: orm.post_id,
            term_id: orm.term_id,
            post_slug: orm.post_slug,
            post_title: orm.post_title,
            term_slug: orm.term_slug,
            term_name: orm.term_name,
            taxonomy_id: orm.taxonomy_id,
            taxonomy_code: orm.taxonomy_code,
        }
    }
}

impl PostTermInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostTermInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_terms_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostTermInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostTermInfoOrm::searchable_columns()
    }
}

impl SqlxViewRepository for PostTermInfoSqlxRepository {
    type Entity = PostTermInfo;
    type Orm = PostTermInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostTermInfo::from(orm)
    }
}

impl PostTermInfoRepository for PostTermInfoSqlxRepository {}
