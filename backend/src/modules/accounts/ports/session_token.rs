use crate::modules::accounts::domain::{SessionToken, SessionTokenHash};

/// Generates cryptographically secure session tokens.
///
/// Generated tokens are intended to be sent to clients as opaque
/// authenticated credentials.
pub trait SessionTokenGenerator: Send + Sync {
    /// Generates a new session token.
    fn generate(&self) -> Result<SessionToken, SessionTokenGeneratorError>;
}

/// Errors returned while generating session tokens.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum SessionTokenGeneratorError {
    #[error("Failed to generate session token")]
    GenerationFailed,
}

/// Produces the persistent representation of session tokens.
///
/// Implementations must be deterministic so the same session token
/// always produces the same hash
pub trait SessionTokenHasher: Send + Sync {
    /// Computes the hash of a session token.
    fn hash(&self, token: &SessionToken) -> SessionTokenHash;
}
