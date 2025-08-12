use crate::business::error::CoreError;
use crate::business::filter::Filter;
use crate::business::repository::Repository;
use crate::business::sort::SortCriterion;
use crate::define_struct_with_common_fields;
use std::sync::Arc;

pub trait UserRepository: Repository<User, UserCreate> + Send + Sync {
    fn find_by_username(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = Result<Option<User>, CoreError>>;
    fn find_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<Option<User>, CoreError>>;
    fn find_by_email_or_username(
        &self,
        email_or_username: &str,
    ) -> impl std::future::Future<Output = Result<Option<User>, CoreError>>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(i32)]
pub enum UserRole {
    USER = 0,
    ADMIN = 1,
}

impl From<i32> for UserRole {
    fn from(role: i32) -> Self {
        match role {
            1 => UserRole::ADMIN,
            _ => UserRole::USER,
        }
    }
}

impl UserRole {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

define_struct_with_common_fields!(User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: UserRole
});

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn get_all(&self, filters: Vec<Filter>) -> Result<Vec<User>, CoreError> {
        self.user_repository.find_all(filters).await
    }
    pub async fn count(&self, filters: Vec<Filter>) -> Result<i64, CoreError> {
        self.user_repository.count(filters).await
    }
    pub async fn get_many(
        &self,
        sort_criteria: Vec<SortCriterion>,
        first_result: Option<i32>,
        max_results: Option<i32>,
        filters: Vec<Filter>,
    ) -> Result<Vec<User>, CoreError> {
        self.user_repository
            .find_many(sort_criteria, first_result, max_results, filters)
            .await
    }
    pub async fn get_by_id(&self, id: i32) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_id(id).await
    }
    pub async fn get_by_uid(&self, uid: String) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_uid(uid).await
    }
    pub async fn create(&self, user_create: &UserCreate) -> Result<User, CoreError> {
        self.user_repository.create(user_create).await
    }
    pub async fn update(&self, user: &User) -> Result<User, CoreError> {
        self.user_repository.update(user).await
    }
    pub async fn delete_by_id(&self, id: i32) -> Result<u64, CoreError> {
        self.user_repository.delete_by_id(id).await
    }
    pub async fn delete_by_uid(&self, uid: String) -> Result<u64, CoreError> {
        self.user_repository.delete_by_uid(uid).await
    }
    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_email(email).await
    }
    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_username(username).await
    }
    pub async fn get_by_email_or_username(&self, email_or_username: &str) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_email_or_username(email_or_username).await
    }
}
