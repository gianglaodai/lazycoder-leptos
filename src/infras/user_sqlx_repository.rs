#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use crate::business::repository::{Repository, SortCriterion};
use crate::business::user_service::{User, UserRepository};
use crate::define_orm_with_common_fields;
use crate::infras::sqlx_repository::SqlxRepository;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserSqlxRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(UserOrm {
    pub username: String,
    pub email: String,
    pub password: String
});

impl From<UserOrm> for User {
    fn from(user: UserOrm) -> Self {
        Self {
            id: user.id,
            uid: user.uid,
            created_at: user.created_at,
            updated_at: user.updated_at,
            username: user.username,
            email: user.email,
            password: user.password,
        }
    }
}

impl UserSqlxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Repository<User> for UserSqlxRepository {
    async fn find_all(&self) -> Result<Vec<User>, CoreError> {
        SqlxRepository::find_all(self).await
    }

    async fn find_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
    ) -> Result<Vec<User>, CoreError> {
        SqlxRepository::find_many(self, sort_criteria, first_result, max_results).await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, CoreError> {
        SqlxRepository::find_by_id(self, id).await
    }

    async fn find_by_uid(&self, uid: Uuid) -> Result<Option<User>, CoreError> {
        SqlxRepository::find_by_uid(self, uid).await
    }

    async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_id(self, id).await
    }

    async fn delete_by_uid(&self, uid: Uuid) -> Result<u64, CoreError> {
        SqlxRepository::delete_by_uid(self, uid).await
    }

    async fn create(&self, user: &User) -> Result<User, CoreError> {
        let now = time::OffsetDateTime::now_utc();

        let user = sqlx::query_as::<_, UserOrm>(
            "INSERT INTO users (uid, username, email, password, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *",
        )
        .bind(Uuid::now_v7())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password)
        .bind(&now)
        .bind(&now)
        .fetch_one(&self.pool)
        .await?;

        Ok(User::from(user))
    }

    async fn update(&self, user: &User) -> Result<User, CoreError> {
        let id = user.id.ok_or_else(|| {
            CoreError::ValidationError("User ID is required for update".to_string())
        })?;
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
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(User::from(user))
    }
}

impl SqlxRepository for UserSqlxRepository {
    type Entity = User;
    type Orm = UserOrm;

    fn get_table_name(&self) -> &str {
        "users"
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
}
