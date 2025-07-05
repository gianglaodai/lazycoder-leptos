#![cfg(feature = "ssr")]

use crate::business::error::CoreError;
use crate::business::user_service::{User, UserRepository};
use crate::define_orm_with_common_fields;
use sqlx::PgPool;

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
impl UserRepository for UserSqlxRepository {
    async fn find_many(&self) -> Result<Vec<User>, CoreError> {
        let users = sqlx::query_as::<_, UserOrm>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;

        Ok(users.into_iter().map(User::from).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, CoreError> {
        let user = sqlx::query_as::<_, UserOrm>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user.map(User::from))
    }

    async fn create(&self, user: &User) -> Result<User, CoreError> {
        let now = time::OffsetDateTime::now_utc();
        let uid = user.uid.unwrap_or_else(|| {
            // Using Uuid::now_v7() for timestamp-based UUID generation
            uuid::Uuid::now_v7()
        });

        let user = sqlx::query_as::<_, UserOrm>(
            "INSERT INTO users (uid, username, email, password, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6) 
             RETURNING *",
        )
        .bind(uid)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password)
        .bind(user.created_at.unwrap_or(now))
        .bind(user.updated_at.unwrap_or(now))
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
        .bind(user.updated_at.unwrap_or(now))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(User::from(user))
    }

    async fn delete(&self, id: i32) -> Result<u64, CoreError> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    async fn find_by_username(&self, name: &str) -> Result<Option<User>, CoreError> {
        let user = sqlx::query_as::<_, UserOrm>("SELECT * FROM users WHERE username = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user.map(User::from))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, CoreError> {
        let user = sqlx::query_as::<_, UserOrm>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user.map(User::from))
    }
}
