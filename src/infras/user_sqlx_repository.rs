#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::Repository;
use crate::business::sort::SortCriterion;
use crate::business::user_service::{User, UserCreate, UserRepository, UserRole};
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserSqlxRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: i32,
});

impl UserOrm {
    pub fn searchable_columns() -> Vec<&'static str> {
        vec!["username", "email"]
    }
}
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

impl UserSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Repository<User, UserCreate> for UserSqlxRepository {
    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<User>, CoreError> {
        SqlxRepository::find_many(self, sort_criteria, first_result, max_results, filters).await
    }

    async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        SqlxRepository::count(self, filters).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, CoreError> {
        SqlxRepository::find_by_id(self, id).await
    }

    async fn find_by_uid(&self, uid: String) -> Result<Option<User>, CoreError> {
        SqlxRepository::find_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }

    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_id(self, id).await
    }

    async fn delete_by_uid(&self, uid: String) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_uid(self, Uuid::parse_str(&uid).unwrap()).await
    }

    async fn create(&self, user_create: &UserCreate) -> Result<User, CoreError> {
        let now = time::OffsetDateTime::now_utc();

        let user = sqlx::query_as::<_, UserOrm>(
            "INSERT INTO users (uid, username, email, password, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *",
        )
        .bind(Uuid::now_v7())
        .bind(&user_create.username)
        .bind(&user_create.email)
        .bind(&user_create.password)
        .bind(&now)
        .bind(&now)
        .fetch_one(&self.pool)
        .await?;

        Ok(User::from(user))
    }

    async fn update(&self, user: &User) -> Result<User, CoreError> {
        let now = time::OffsetDateTime::now_utc();

        let user = sqlx::query_as::<_, UserOrm>(
            "UPDATE users
             SET username = $1, email = $2, password = $3, updated_at = $4
             WHERE id = $5
             RETURNING *",
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&now)
        .bind(user.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(User::from(user))
    }
}

impl SqlxRepository for UserSqlxRepository {
    type Entity = User;
    type EntityCreate = UserCreate;
    type Orm = UserOrm;

    fn get_table_name(&self) -> &str {
        "users"
    }

    fn get_columns(&self) -> Vec<&str> {
        UserOrm::columns()
    }

    fn get_searchable_columns(&self) -> Vec<&str> {
        UserOrm::searchable_columns()
    }

    fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    fn from_orm(orm: Self::Orm) -> Self::Entity {
        User::from(orm)
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

    async fn find_by_email_or_username(&self, email_or_username: &str) -> Result<Option<User>, CoreError> {
        let result = sqlx::query_as::<_, UserOrm>("SELECT * FROM users WHERE email = $1 OR username = $1")
            .bind(email_or_username)
            .fetch_optional(self.get_pool())
            .await?;
        Ok(result.map(Self::from_orm))
    }
}
