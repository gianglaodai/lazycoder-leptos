#![cfg(feature = "ssr")]

use crate::business::post_type_service::{
    PostType, PostTypeCreate, PostTypeInfo, PostTypeInfoRepository, PostTypeRepository,
};
use crate::infras::sqlx_repository::{
    SqlxEntityMapper, SqlxRepository, SqlxViewMeta, SqlxViewRepository,
};
use crate::{define_orm_with_common_fields, define_readonly_orm_with_common_fields};
use sqlx::PgPool;
use uuid::Uuid;

define_orm_with_common_fields!(PostType {
    pub code: String,
    pub name: String,
});

define_readonly_orm_with_common_fields!(PostTypeInfo {
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

#[derive(Clone)]
pub struct PostTypeSqlxRepository {
    pool: PgPool,
}
#[derive(Clone)]
pub struct PostTypeInfoSqlxRepository {
    pool: PgPool,
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
