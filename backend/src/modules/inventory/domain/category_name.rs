#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CategoryName(String);

impl CategoryName {
    pub fn parse(value: &str) -> Result<Self, CategoryNameError> {
        let value = value.split_whitespace().collect::<Vec<_>>().join(" ");

        if value.is_empty() {
            return Err(CategoryNameError::Empty);
        }

        if value.chars().count() > 50 {
            return Err(CategoryNameError::TooLong);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn normalized(&self) -> String {
        self.0.to_lowercase()
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CategoryNameError {
    #[error("Category name cannot be empty")]
    Empty,
    #[error("Category name cannot be longer than 50 characters")]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name_is_accepted() {
        let name = CategoryName::parse("Food").expect("Name should be valid");

        assert_eq!(name.as_str(), "Food")
    }

    #[test]
    fn repeated_whitespace_is_normalized() {
        let name = CategoryName::parse("More     Food").expect("Name should be valid");

        assert_eq!(name.as_str(), "More Food")
    }

    #[test]
    fn normalized_name_is_case_insensitive() {
        let name = CategoryName::parse("Food").expect("Name should be valid");

        assert_eq!(name.normalized(), "food")
    }

    #[test]
    fn surrounding_whitespace_is_trimmed() {
        let name = CategoryName::parse("      Food      ").expect("Name should be valid");

        assert_eq!(name.as_str(), "Food")
    }

    #[test]
    fn empty_name_is_rejected() {
        assert_eq!(CategoryName::parse(""), Err(CategoryNameError::Empty))
    }

    #[test]
    fn whitespace_only_name_is_rejected() {
        assert_eq!(
            CategoryName::parse("             "),
            Err(CategoryNameError::Empty)
        )
    }

    #[test]
    fn too_long_name_gets_rejected() {
        assert_eq!(
            CategoryName::parse("!".repeat(51).as_str()),
            Err(CategoryNameError::TooLong)
        )
    }
}
