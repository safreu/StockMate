use core::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn parse(value: &str) -> Result<Self, PasswordHashError> {
        let value = value.to_owned();
        if value.trim().is_empty() {
            return Err(PasswordHashError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PasswordHash([REDACTED])")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordHashError {
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty_hash_is_accepted() {
        let hash = PasswordHash::parse("$argon2id$example");

        assert!(hash.is_ok());
    }

    #[test]
    fn empty_hash_is_rejected() {
        let result = PasswordHash::parse("");

        assert_eq!(result, Err(PasswordHashError::Empty));
    }
    #[test]
    fn whitespace_only_hash_is_rejected() {
        let result = PasswordHash::parse("      ");

        assert_eq!(result, Err(PasswordHashError::Empty));
    }
    #[test]
    fn hash_can_be_read_as_str() {
        let hash = PasswordHash::parse("$argon2id$example").expect("Hash should be valid");

        assert_eq!(hash.as_str(), "$argon2id$example");
    }
}
