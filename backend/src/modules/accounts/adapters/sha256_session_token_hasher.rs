use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

use crate::modules::accounts::{
    domain::{SessionToken, SessionTokenHash},
    ports::SessionTokenHasher,
};

pub struct Sha256SessionTokenHasher;

impl Sha256SessionTokenHasher {
    pub fn new() -> Self {
        Self
    }
}

impl SessionTokenHasher for Sha256SessionTokenHasher {
    fn hash(&self, token: &SessionToken) -> SessionTokenHash {
        let digest = Sha256::digest(token.as_str().as_bytes());
        let encoded = URL_SAFE_NO_PAD.encode(digest);

        SessionTokenHash::from_encoded(&encoded)
            .expect("SHA-256 output encoding must be valid token hash")
    }
}

impl Default for Sha256SessionTokenHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(value: &str) -> SessionToken {
        SessionToken::from_string(value.to_owned())
    }

    #[test]
    fn same_token_produces_same_hash() {
        let hasher = Sha256SessionTokenHasher;

        let token = token("this-is-a-session-token");

        let first = hasher.hash(&token);
        let second = hasher.hash(&token);

        assert_eq!(first, second)
    }

    #[test]
    fn different_tokens_produces_different_hashes() {
        let hasher = Sha256SessionTokenHasher;

        let first = hasher.hash(&token("this-is-a-session-token"));
        let second = hasher.hash(&token("this-is-another-session-token"));

        assert_ne!(first, second)
    }

    #[test]
    fn hash_does_not_contain_raw_token() {
        let hasher = Sha256SessionTokenHasher;

        let token = token("this-is-a-session-token");

        let hash = hasher.hash(&token);

        assert_ne!(hash.as_str(), token.as_str())
    }
}
