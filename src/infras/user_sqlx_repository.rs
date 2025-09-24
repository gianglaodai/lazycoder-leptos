#![cfg(feature = "ssr")]

use crate::business::user_service::{
    User, UserCreate, UserInfo, UserInfoRepository, UserRepository, UserRole,
};
use crate::common::error::CoreError;
use crate::infras::sqlx_repository::{
    SqlxEntityMapper, SqlxRepository, SqlxViewMeta, SqlxViewRepository,
};
use crate::{define_orm_with_common_fields, define_readonly_orm_with_common_fields};
use sqlx::PgPool;
use uuid::Uuid;

define_orm_with_common_fields!(User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: i32,
});

define_readonly_orm_with_common_fields!(UserInfo {
    pub username: String,
    pub email: String,
    pub role: String,
});

impl From<UserOrm> for User {
    fn from(orm: UserOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            username: orm.username,
            email: orm.email,
            password: orm.password,
            role: UserRole::from(orm.role),
        }
    }
}

impl From<UserInfoOrm> for UserInfo {
    fn from(orm: UserInfoOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid.to_string(),
            version: orm.version,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            username: orm.username,
            email: orm.email,
            role: orm.role,
        }
    }
}

#[derive(Clone)]
pub struct UserSqlxRepository {
    pool: PgPool,
}
#[derive(Clone)]
pub struct UserInfoSqlxRepository {
    pool: PgPool,
}

impl UserInfoSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["username", "email"]
    }
}
impl UserSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxViewMeta for UserSqlxRepository {
    fn get_table_name(&self) -> &str {
        "users"
    }
    fn get_columns(&self) -> Vec<&str> {
        UserOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        UserOrm::searchable_columns()
    }
}

impl SqlxViewRepository for UserSqlxRepository {
    type Entity = User;
    type Orm = UserOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        User::from(orm)
    }
}

impl SqlxEntityMapper for UserSqlxRepository {
    type Entity = User;
    type EntityCreate = UserCreate;
    type Orm = UserOrm;

    fn to_orm_from_create(&self, create: &Self::EntityCreate) -> Self::Orm {
        let now = time::OffsetDateTime::now_utc();
        UserOrm {
            id: 0,
            uid: Uuid::now_v7(),
            version: 0,
            created_at: now,
            updated_at: now,
            username: create.username.clone(),
            email: create.email.clone(),
            password: create.password.clone(),
            role: UserRole::USER.as_i32(),
        }
    }

    fn to_orm_from_entity(&self, entity: &Self::Entity) -> Self::Orm {
        UserOrm {
            id: entity.id,
            uid: Uuid::parse_str(&entity.uid).unwrap_or_else(|_| Uuid::nil()),
            version: entity.version,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            username: entity.username.clone(),
            email: entity.email.clone(),
            password: entity.password.clone(),
            role: entity.role.as_i32(),
        }
    }
}

impl SqlxRepository for UserSqlxRepository {
    type EntityCreate = UserCreate;
}

impl SqlxViewMeta for UserInfoSqlxRepository {
    fn get_table_name(&self) -> &str {
        "users_info"
    }
    fn get_columns(&self) -> Vec<&str> {
        UserInfoOrm::columns()
    }
    fn get_searchable_columns(&self) -> Vec<&str> {
        vec!["username", "email", "role"]
    }
}

impl SqlxViewRepository for UserInfoSqlxRepository {
    type Entity = UserInfo;
    type Orm = UserInfoOrm;
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
    fn from_orm(orm: Self::Orm) -> Self::Entity {
        UserInfo::from(orm)
    }
}

impl UserRepository for UserSqlxRepository {
    async fn find_by_username(&self, name: &str) -> Result<Option<User>, CoreError> {
        let result = sqlx::query_as::<_, UserOrm>("SELECT * FROM users WHERE username = $1")
            .bind(name)
            .fetch_optional(self.get_pool())
            .await?;
        Ok(result.map(Self::from_orm))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, CoreError> {
        let result = sqlx::query_as::<_, UserOrm>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(self.get_pool())
            .await?;
        Ok(result.map(Self::from_orm))
    }

    async fn find_by_email_or_username(
        &self,
        email_or_username: &str,
    ) -> Result<Option<User>, CoreError> {
        let result =
            sqlx::query_as::<_, UserOrm>("SELECT * FROM users WHERE email = $1 OR username = $1")
                .bind(email_or_username)
                .fetch_optional(self.get_pool())
                .await?;
        Ok(result.map(Self::from_orm))
    }
}

impl UserInfoRepository for UserInfoSqlxRepository {}
