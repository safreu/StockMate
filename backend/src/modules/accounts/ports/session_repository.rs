use async_trait::async_trait;

use crate::modules::accounts::domain::Session;
use crate::modules::accounts::domain::SessionTokenHash;

/// Persists and retrieves authentication sessions.
///
/// Raw session tokens must never be stored directly.
/// Instead sessions are identified by the hash of their token.
#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// Persists a new session
    /// Return `TokenHashAlreadyExists` if another session already
    /// uses the same token hash.
    async fn insert(&self, session: &Session) -> Result<(), SessionRepositoryError>;

    /// Returns the session associated with the given token hash,
    /// or `None` if not matching session exists.
    async fn find_by_token_hash(
        &self,
        token_hash: &SessionTokenHash,
    ) -> Result<Option<Session>, SessionRepositoryError>;

    /// Deletes the session associated with the given token hash,
    /// Deleting a non-existent session is treated as success.
    async fn delete_by_token_hash(
        &self,
        token_hash: &SessionTokenHash,
    ) -> Result<(), SessionRepositoryError>;
}

/// Errors returned by `SessionRepository` implementations.
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
