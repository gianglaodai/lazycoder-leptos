#![cfg(feature = "ssr")]

use crate::business::user_service::{User, UserCreate, UserRepository, UserRole, UserService};
use crate::common::error::CoreError;

#[derive(Clone)]
pub struct AuthService<R: UserRepository> {
    user_service: UserService<R>,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(user_service: UserService<R>) -> Self {
        Self { user_service }
    }

    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, CoreError> {
        // Check existing username/email
        if let Some(_) = self.user_service.get_by_username(&username).await? {
            return Err(CoreError::conflict("error.username.registered"));
        }
        if let Some(_) = self.user_service.get_by_email(&email).await? {
            return Err(CoreError::conflict("error.email.registered"));
        }

        log::info!("password: {}", &password);
        let hashed = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
            .map_err(|_| CoreError::internal_server_error("error.encrypt.unknown"))?;

        let user_create = UserCreate {
            username,
            email,
            password: hashed,
            role: UserRole::USER,
        };

        self.user_service.create(&user_create).await
    }

    pub async fn login(
        &self,
        username_or_email: String,
        password: String,
    ) -> Result<User, CoreError> {
        let user_opt = self
            .user_service
            .get_by_email_or_username(&username_or_email)
            .await?;

        let user = match user_opt {
            Some(u) => u,
            None => {
                return Err(CoreError::unauthorized("invalid.credentials"));
            }
        };

        let is_valid = bcrypt::verify(&password, &user.password).unwrap_or(false);

        if !is_valid {
            return Err(CoreError::unauthorized("invalid.credentials"));
        }

        Ok(user)
    }
}
