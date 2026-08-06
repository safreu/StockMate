use crate::modules::accounts::domain::{SessionToken, SessionTokenHash};

pub trait SessionTokenGenerator: Send + Sync {
    fn generate(&self) -> Result<SessionToken, SessionTokenGeneratorError>;
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum SessionTokenGeneratorError {
    #[error("Failed to generate session token")]
    GenerationFailed,
}

pub trait SessionTokenHasher: Send + Sync {
    fn hash(&self, token: &SessionToken) -> SessionTokenHash;
}
