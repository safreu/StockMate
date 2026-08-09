#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum InternalError {
    #[error("internal operation failed")]
    Failed,
}
