use core::fmt;

use email_address::EmailAddress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, EmailError> {
        let email = email.trim().to_owned();
        if email.is_empty() {
            return Err(EmailError::Empty);
        }
        if !EmailAddress::is_valid(&email) {
            return Err(EmailError::InvalidFormat);
        }

        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailError {
    Empty,
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email_is_accepted() {
        let result = Email::parse("example.email@email.com");

        assert!(result.is_ok());
    }

    #[test]
    fn surrounding_whitespace_is_removed() {
        let result = Email::parse("   example.email@email.com    ").expect("email should be valid");

        assert_eq!(result.as_str(), "example.email@email.com");
    }

    #[test]
    fn empty_email_is_rejected() {
        let result = Email::parse("");

        assert_eq!(result, Err(EmailError::Empty));
    }

    #[test]
    fn whitespace_only_email_is_rejected() {
        let result = Email::parse("    ");

        assert_eq!(result, Err(EmailError::Empty));
    }

    #[test]
    fn email_without_at_sign_is_rejected() {
        let result = Email::parse("example.emailemail.com");

        assert_eq!(result, Err(EmailError::InvalidFormat));
    }

    #[test]
    fn email_with_multiple_at_signs_is_rejected() {
        let result = Email::parse("example.email@@email.com");

        assert_eq!(result, Err(EmailError::InvalidFormat));
    }

    #[test]
    fn email_without_domain_is_rejected() {
        let result = Email::parse("example.email@");

        assert_eq!(result, Err(EmailError::InvalidFormat));
    }

    #[test]
    fn email_without_local_part_is_rejected() {
        let result = Email::parse("@email.com");

        assert_eq!(result, Err(EmailError::InvalidFormat));
    }

    #[test]
    fn email_with_internal_whitespace_is_rejected() {
        let result = Email::parse("example email@email.com");

        assert_eq!(result, Err(EmailError::InvalidFormat));
    }
}
