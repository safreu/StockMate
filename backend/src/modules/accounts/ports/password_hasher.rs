use crate::modules::accounts::domain::PasswordHash;

/// Hashes and verifies user passwords.
///
/// Implementations are expected to use a secure password hashing algorithm
/// such as Argon2.
pub trait PasswordHasher: Send + Sync {
    /// Computes a password hash suitable for persistent storage.
    fn hash(&self, password: &str) -> Result<PasswordHash, PasswordHasherError>;

    /// Verifies a password against its stored hash.
    ///
    /// Returns `Ok(false)` when the password does not match.
    /// Returns `Err(_)` only for technical failures.
    fn verify(&self, password: &str, hash: &PasswordHash) -> Result<bool, PasswordHasherError>;
}

/// Errors returned while hashing or verifying passwords.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum PasswordHasherError {
    #[error("Password hashing failed")]
    HashFailed,
    #[error("Password verification failed")]
    VerifyFailed,
}
