use async_trait::async_trait;

use crate::modules::accounts::domain::Session;
use crate::modules::accounts::domain::SessionTokenHash;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn insert(&self, session: &Session) -> Result<(), SessionRepositoryError>;

    async fn find_by_token(
        &self,
        token_hash: &SessionTokenHash,
    ) -> Result<Option<Session>, SessionRepositoryError>;

    async fn delete_by_token_hash(
        &self,
        token_hash: &SessionTokenHash,
    ) -> Result<(), SessionRepositoryError>;
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum SessionRepositoryError {
    #[error("Database operation failed")]
    Database,
    #[error("Stored session data is invalid")]
    InvalidStoredData,
    #[error("Database is unavailable")]
    Unavailable,
    #[error("Session token hash already exists")]
    TokenHashAlreadyExists,
}
