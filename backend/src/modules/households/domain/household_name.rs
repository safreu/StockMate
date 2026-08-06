#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseholdName(String);

impl HouseholdName {
    pub fn parse(value: &str) -> Result<Self, HouseholdNameError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(HouseholdNameError::Empty);
        };

        if value.chars().count() > 100 {
            return Err(HouseholdNameError::TooLong);
        }

        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HouseholdNameError {
    #[error("Household name cannot be empty")]
    Empty,
    #[error("Household name cannot be longer than 50 characters")]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surrounding_whitespace_is_removed() {
        let name = "    household_name      ";
        let result = HouseholdName::parse(name).expect("Household name should be parsable");

        assert_eq!(result.as_str(), "household_name")
    }

    #[test]
    fn empty_household_name_is_rejected() {
        let name = " ";
        let result = HouseholdName::parse(name);

        assert_eq!(result, Err(HouseholdNameError::Empty))
    }

    #[test]
    fn too_long_household_name_is_rejected() {
        let name = &"a".repeat(101);
        let result = HouseholdName::parse(name);

        assert_eq!(result, Err(HouseholdNameError::TooLong))
    }

    #[test]
    fn valid_household_name_is_accepted() {
        let name = "this is a household display name";
        let result = HouseholdName::parse(name);

        assert!(result.is_ok())
    }
}
