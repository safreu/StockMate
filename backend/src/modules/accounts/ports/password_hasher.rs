use crate::modules::accounts::domain::PasswordHash;

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<PasswordHash, PasswordHasherError>;

    fn verify(&self, password: &str, hash: &PasswordHash) -> Result<bool, PasswordHasherError>;
}

//TODO: Update the other errors with this::Error and custom error messages to keep consistency
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum PasswordHasherError {
    #[error("Password hashing failed")]
    HashingFailed,
    #[error("Password verification failed")]
    VerificationFailed,
}
