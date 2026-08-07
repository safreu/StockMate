#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PersistenceError {
    #[error("Persistence Layer is unavailable")]
    Unavailable,
    #[error("Persistence Operation failed")]
    Failed,
}

pub fn map_sqlx_error(error: sqlx::Error) -> PersistenceError {
    tracing::error!(
        error = ?error,
        "database operation failed"
    );

    match error {
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::Io(_) => {
            PersistenceError::Unavailable
        }
        _ => PersistenceError::Failed,
    }
}
