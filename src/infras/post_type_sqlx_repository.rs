#![cfg(feature = "ssr")]

use crate::business::post_type_service::{PostType, PostTypeCreate, PostTypeRepository};
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::{
    SqlxEntityMapper, SqlxRepository, SqlxViewMeta, SqlxViewRepository,
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostTypeSqlxRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(PostType {
    pub code: String,
    pub name: String,
});

impl PostTypeOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["code", "name"]
    }
}

impl From<PostTypeOrm> for PostType {
    fn from(orm: PostTypeOrm) -> Self {
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

impl PostTypeSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for PostTypeSqlxRepository {
    fn get_table_name(&self) -> &str {
        "post_types"
    }
    fn get_columns(&self) -> Vec<&str> {
        PostTypeOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        PostTypeOrm::searchable_columns()
    }
}

impl SqlxEntityMapper for PostTypeSqlxRepository {
    type Entity = PostType;
    type EntityCreate = PostTypeCreate;
    type Orm = PostTypeOrm;

    fn to_orm_from_create(&self, create: &Self::EntityCreate) -> Self::Orm {
        let now = time::OffsetDateTime::now_utc();
        PostTypeOrm {
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
        PostTypeOrm {
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

impl SqlxRepository for PostTypeSqlxRepository {
    type EntityCreate = PostTypeCreate;
}

impl SqlxViewRepository for PostTypeSqlxRepository {
    type Entity = PostType;
    type Orm = PostTypeOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        PostType::from(orm)
    }
}

impl PostTypeRepository for PostTypeSqlxRepository {}
