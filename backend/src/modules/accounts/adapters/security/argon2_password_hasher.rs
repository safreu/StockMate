use argon2::{
    Argon2,
    password_hash::{
        Error as Argon2Error, PasswordHash as Argon2Hash, PasswordHasher as Argon2Hasher,
        PasswordVerifier, SaltString, rand_core::OsRng,
    },
};

use crate::modules::accounts::domain::PasswordHash;
use crate::modules::accounts::ports::{PasswordHasher, PasswordHasherError};

pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, password: &str) -> Result<PasswordHash, PasswordHasherError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let encoded_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| PasswordHasherError::HashFailed)?
            .to_string();

        PasswordHash::from_encoded(&encoded_hash).map_err(|_| PasswordHasherError::HashFailed)
    }

    fn verify(&self, password: &str, hash: &PasswordHash) -> Result<bool, PasswordHasherError> {
        let parsed_hash =
            Argon2Hash::new(hash.as_str()).map_err(|_| PasswordHasherError::VerifyFailed)?;

        match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(Argon2Error::Password) => Ok(false),
            Err(_) => Err(PasswordHasherError::VerifyFailed),
        }
    }
}

impl Default for Argon2PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_can_be_hashed() {
        let password = "This is a secret password";
        let hasher = Argon2PasswordHasher::new();

        let hashed_password = hasher.hash(password);

        assert!(hashed_password.is_ok())
    }

    #[test]
    fn correct_password_is_accepted() {
        let password = "This is a secret password";
        let hasher = Argon2PasswordHasher::new();

        let hashed_password = hasher.hash(password).expect("Password should be hashable");

        assert!(
            hasher
                .verify(password, &hashed_password)
                .expect("Password verification should succeed")
        );
    }

    #[test]
    fn incorrect_password_is_rejected() {
        let password = "This is a secret password";
        let hasher = Argon2PasswordHasher;

        let hashed_password = hasher.hash(password).expect("Password should be hashable");

        assert!(
            !hasher
                .verify("This is another secret password", &hashed_password)
                .expect("Password verification should not succeed")
        );
    }

    #[test]
    fn identical_passwords_receive_different_hashes() {
        let password = "This is a secret password";
        let hasher = Argon2PasswordHasher;

        let hashed_password = hasher.hash(password).expect("Password should be hashable");
        let another_hashed_password = hasher.hash(password).expect("Password should be hashable");

        assert_ne!(hashed_password, another_hashed_password);
    }

    #[test]
    fn malformed_hash_returns_verification_error() {
        let password = "This is a secret password";
        let hasher = Argon2PasswordHasher;

        let malformed_hash =
            PasswordHash::from_encoded("not-an-argon2-hash").expect("Value is non-empty");

        let result = hasher.verify(password, &malformed_hash);

        assert_eq!(result, Err(PasswordHasherError::VerifyFailed));
    }
}
