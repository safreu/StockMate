use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::{TryRngCore, rngs::OsRng};

use crate::modules::accounts::{
    domain::SessionToken,
    ports::{SessionTokenGenerator, SessionTokenGeneratorError},
};

pub struct SecureSessionTokenGenerator;

impl SecureSessionTokenGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl SessionTokenGenerator for SecureSessionTokenGenerator {
    fn generate(&self) -> Result<SessionToken, SessionTokenGeneratorError> {
        let mut bytes = [0_u8; 32];

        OsRng
            .try_fill_bytes(&mut bytes)
            .map_err(|_| SessionTokenGeneratorError::GenerationFailed)?;

        let encoded = URL_SAFE_NO_PAD.encode(bytes);

        SessionToken::from_string(encoded).map_err(|_| SessionTokenGeneratorError::GenerationFailed)
    }
}

impl Default for SecureSessionTokenGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_token_is_not_empty() {
        let generator = SecureSessionTokenGenerator;

        let token = generator
            .generate()
            .expect("Token generation should succeed");

        assert!(!token.as_str().is_empty())
    }

    #[test]
    fn generated_tokens_are_different() {
        let generator = SecureSessionTokenGenerator;

        let first = generator
            .generate()
            .expect("First token generation should succeed");

        let second = generator
            .generate()
            .expect("Second token generation should succeed");

        assert_ne!(first, second)
    }
}
