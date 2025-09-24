#![cfg(feature = "ssr")]

use crate::business::post_relation_service::{
    PostRelation, PostRelationInfo, PostRelationInfoRepository, PostRelationRepository,
};
use crate::infras::sqlx_repository::{SqlxViewMeta, SqlxViewRepository};
use crate::{define_orm_with_common_fields, define_readonly_orm_with_common_fields};
use sqlx::PgPool;

define_orm_with_common_fields!(PostRelation {
    pub from_post: i32,
    pub to_post: i32,
    pub rel_type: String,
});
define_readonly_orm_with_common_fields!(PostRelationInfo {
    pub from_post: i32,
    pub to_post: i32,
    pub rel_type: String,
    pub from_slug: Option<String>,
    pub from_title: Option<String>,
    pub to_slug: Option<String>,
    pub to_title: Option<String>,
});
impl From<PostRelationOrm> for PostRelation {
    fn from(orm: PostRelationOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            from_post: orm.from_post,
            to_post: orm.to_post,
            rel_type: orm.rel_type,
        }
    }
}
impl From<PostRelationInfoOrm> for PostRelationInfo {
    fn from(orm: PostRelationInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            from_post: orm.from_post,
            to_post: orm.to_post,
            rel_type: orm.rel_type,
            from_slug: orm.from_slug,
            from_title: orm.from_title,
            to_slug: orm.to_slug,
            to_title: orm.to_title,
        }
    }
}
#[derive(Clone)]
pub struct PostRelationSqlxRepository {
    pool: PgPool,
}
#[derive(Clone)]
pub struct PostRelationInfoSqlxRepository {
    pool: PgPool,
}

impl PostRelationOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["rel_type"]
    }
}

impl PostRelationSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostRelationSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_relations"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostRelationOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostRelationOrm::searchable_columns()
    }
}

impl SqlxViewRepository for PostRelationSqlxRepository {
    type Entity = PostRelation;
    type Orm = PostRelationOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostRelation::from(orm)
    }
}

impl PostRelationRepository for PostRelationSqlxRepository {}

impl PostRelationInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["rel_type", "from_slug", "to_slug", "from_title", "to_title"]
    }
}

impl PostRelationInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostRelationInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_relations_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostRelationInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostRelationInfoOrm::searchable_columns()
    }
}

impl SqlxViewRepository for PostRelationInfoSqlxRepository {
    type Entity = PostRelationInfo;
    type Orm = PostRelationInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostRelationInfo::from(orm)
    }
}

impl PostRelationInfoRepository for PostRelationInfoSqlxRepository {}
