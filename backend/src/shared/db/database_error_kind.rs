#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseErrorKind {
    Unavailable,
    Other,
}

pub fn classify_sqlx_error(error: &sqlx::Error) -> DatabaseErrorKind {
    match error {
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::Io(_) => {
            DatabaseErrorKind::Unavailable
        }
        _ => DatabaseErrorKind::Other,
    }
}
