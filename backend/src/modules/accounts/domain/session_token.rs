use core::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct SessionToken(String);

impl SessionToken {
    pub fn from_string(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Debug for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SessionToken([REDACTED])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string_preserves_value() {
        let token = "This is a session token";
        let session_token = SessionToken::from_string(token.to_string());

        assert_eq!(session_token.as_str(), token)
    }

    #[test]
    fn into_string_returns_inner_string() {
        let token = "This is a session token";
        let session_token = SessionToken::from_string(token.to_string());

        assert_eq!(session_token.into_string(), token)
    }
}
