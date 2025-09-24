#![cfg(feature = "ssr")]

use crate::business::post_service::{PostInfo, PostInfoRepository, PostStatus};
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::{SqlxViewMeta, SqlxViewRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostInfoSqlxRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(PostInfo {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub status: i32,
    pub user_id: i32,
    pub username: String,
    pub email: String,
});

impl From<PostInfoOrm> for PostInfo {
    fn from(orm: PostInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            slug: orm.slug,
            title: orm.title,
            summary: orm.summary,
            content: orm.content,
            status: PostStatus::from(orm.status),
            user_id: orm.user_id,
            username: orm.username,
            email: orm.email,
        }
    }
}

impl PostInfoOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["slug", "title", "summary", "content"]
    }
}

impl PostInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "posts_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostInfoOrm::searchable_columns()
    }
}

impl SqlxViewRepository for PostInfoSqlxRepository {
    type Entity = PostInfo;
    type Orm = PostInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostInfo::from(orm)
    }
}

impl PostInfoRepository for PostInfoSqlxRepository {}
