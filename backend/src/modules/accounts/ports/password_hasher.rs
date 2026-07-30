use crate::modules::accounts::domain::PasswordHash;

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<PasswordHash, PasswordHasherError>;

    fn verify(&self, password: &str, hash: PasswordHash) -> Result<bool, PasswordHasherError>;
}

//TODO: Update the other erros with this::Error and custom error messages to keep consistency
#[derive(Debug, thiserror::Error)]
pub enum PasswordHasherError {
    #[allow(dead_code)]
    #[error("Password hashing failed")]
    HashingFailed,
    #[allow(dead_code)]
    #[error("Password verification failed")]
    VerificationFailed,
}
