#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayName(String);

impl DisplayName {
    pub fn parse(value: &str) -> Result<Self, DisplayNameError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(DisplayNameError::Empty);
        };

        if value.chars().count() > 50 {
            return Err(DisplayNameError::TooLong);
        }

        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DisplayNameError {
    #[error("Display name cannot be empty")]
    Empty,
    #[error("Display name cannot be longer than 50 characters")]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surrounding_whitespace_is_removed() {
        let name = "    user_name      ";
        let result = DisplayName::parse(name).expect("User name should be parsable");

        assert_eq!(result.as_str(), "user_name")
    }

    #[test]
    fn empty_display_name_is_rejected() {
        let name = " ";
        let result = DisplayName::parse(name);

        assert_eq!(result, Err(DisplayNameError::Empty))
    }

    #[test]
    fn too_long_display_name_is_rejected() {
        let name = &"a".repeat(101);
        let result = DisplayName::parse(name);

        assert_eq!(result, Err(DisplayNameError::TooLong))
    }

    #[test]
    fn valid_display_name_is_accepted() {
        let name = "this is a valid display name";
        let result = DisplayName::parse(name);

        assert!(result.is_ok())
    }
}
