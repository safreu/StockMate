#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryItemName(String);

impl InventoryItemName {
    pub fn parse(value: &str) -> Result<Self, InventoryItemNameError> {
        let value = value.split_whitespace().collect::<Vec<_>>().join(" ");

        if value.is_empty() {
            return Err(InventoryItemNameError::Empty);
        }

        if value.chars().count() > 50 {
            return Err(InventoryItemNameError::TooLong);
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
pub enum InventoryItemNameError {
    #[error("Inventory item name cannot be empty")]
    Empty,
    #[error("Inventory name cannot be longer than 50 characters")]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name_is_accepted() {
        let name = InventoryItemName::parse("Tofu").expect("Name should be valid");

        assert_eq!(name.as_str(), "Tofu")
    }

    #[test]
    fn repeated_whitespace_is_normalized() {
        let name = InventoryItemName::parse("More     Tofu").expect("Name should be valid");

        assert_eq!(name.as_str(), "More Tofu")
    }

    #[test]
    fn normalized_name_is_case_insensitive() {
        let name = InventoryItemName::parse("Tofu").expect("Name should be valid");

        assert_eq!(name.normalized(), "tofu")
    }

    #[test]
    fn surrounding_whitespace_is_trimmed() {
        let name = InventoryItemName::parse("      Tofu      ").expect("Name should be valid");

        assert_eq!(name.as_str(), "Tofu")
    }

    #[test]
    fn empty_name_is_rejected() {
        assert_eq!(
            InventoryItemName::parse(""),
            Err(InventoryItemNameError::Empty)
        )
    }

    #[test]
    fn whitespace_only_name_is_rejected() {
        assert_eq!(
            InventoryItemName::parse("             "),
            Err(InventoryItemNameError::Empty)
        )
    }

    #[test]
    fn too_long_name_gets_rejected() {
        assert_eq!(
            InventoryItemName::parse("!".repeat(51).as_str()),
            Err(InventoryItemNameError::TooLong)
        )
    }
}
