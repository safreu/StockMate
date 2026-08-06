use crate::modules::accounts::domain::{Email, User, UserId};
use async_trait::async_trait;

/// Persists and retrieves users.
///
/// Implementations must enforce email uniqueness.
///
/// Repository implementations should never expose implementation-specific
/// persistence errors directly. They mus translate them into `UserRepositoryError`
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Persists a new user
    /// Returns a `EmailAlreadyExists` if another user with the same email exists.
    async fn insert(&self, user: &User) -> Result<(), UserRepositoryError>;

    /// Returns the user with the given id, or `None` if no such user exists.
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError>;

    /// Return the user with the given email, or `None` if no such user exists.
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError>;
}

/// Errors returned by `UserRepository` implementations.
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
