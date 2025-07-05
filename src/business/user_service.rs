use crate::define_struct_with_common_fields;
use std::sync::Arc;
use crate::business::error::CoreError;

pub trait UserRepository: Send + Sync {
    async fn find_many(&self) -> Result<Vec<User>, CoreError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, CoreError>;

    async fn create(&self, user: &User) -> Result<User, CoreError>;
    async fn update(&self, user: &User) -> Result<User, CoreError>;
    async fn delete(&self, id: i32) -> Result<u64, CoreError>;
    async fn find_by_username(&self, name: &str) -> Result<Option<User>, CoreError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, CoreError>;
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

    pub async fn get_many(&self) -> Result<Vec<User>, CoreError> {
        self.user_repository.find_many().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_id(id).await
    }

    pub async fn create(&self, user: &User) -> Result<User, CoreError> {
        self.user_repository.create(user).await
    }

    pub async fn update(&self, user: &User) -> Result<User, CoreError> {
        self.user_repository.update(user).await
    }

    pub async fn delete(&self, id: i32) -> Result<u64, CoreError> {
        self.user_repository.delete(id).await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_email(email).await
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, CoreError> {
        self.user_repository.find_by_username(username).await
    }
}
