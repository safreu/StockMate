use crate::modules::accounts::domain::{Email, User, UserId};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn insert(&self, user: &User) -> Result<(), UserRepositoryError>;
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum UserRepositoryError {
    EmailAlreadyExists,
    Unavailable,
    Unexpected,
}
