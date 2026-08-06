use crate::modules::accounts::domain::{Email, User, UserId};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn insert(&self, user: &User) -> Result<(), UserRepositoryError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError>;
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum UserRepositoryError {
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Database operation failed")]
    Database,
    #[error("Stored user data is invalid")]
    InvalidStoredData,
    #[error("Database is unavailable")]
    Unavailable,
}
