use crate::business::error::CoreError;
use crate::business::repository::Repository;
use crate::define_struct_with_common_fields;
use std::sync::Arc;
use uuid::Uuid;

pub trait UserRepository: Repository<User> + Send + Sync {
    fn find_by_username(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = Result<Option<User>, CoreError>>;
    fn find_by_email(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<Option<User>, CoreError>>;
}

define_struct_with_common_fields!(User {
    pub username: String,
    pub email: String,
    pub password: String
});

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    user_repository: Arc<R>,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(user_repository: Arc<R>) -> Self {
        Self { user_repository }
    }

    pub async fn get_all(&self) -> Result<Vec<User>, CoreError> {
        self.user_repository.find_all().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_id(id).await
    }

    pub async fn get_by_uid(&self, uid: Uuid) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_uid(uid).await
    }

    pub async fn create(&self, user: &User) -> Result<User, CoreError> {
        self.user_repository.create(user).await
    }

    pub async fn update(&self, user: &User) -> Result<User, CoreError> {
        self.user_repository.update(user).await
    }

    pub async fn delete(&self, id: i32) -> Result<u64, CoreError> {
        self.user_repository.delete_by_id(id).await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_email(email).await
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_username(username).await
    }
}
