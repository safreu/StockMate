use std::{fmt, str};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InventoryPriority {
    #[default]
    Default,
    Low,
    Medium,
    High,
}

impl InventoryPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    pub fn parse(value: &str) -> Result<Self, InventoryPriorityError> {
        match value {
            "default" => Ok(Self::Default),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err(InventoryPriorityError::Invalid),
        }
    }
}

impl fmt::Display for InventoryPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl str::FromStr for InventoryPriority {
    type Err = InventoryPriorityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum InventoryPriorityError {
    #[error("Invalid inventory priority")]
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_strings_parse_correctly() {
        assert_eq!(
            InventoryPriority::parse("low").expect("Priority should be valid"),
            InventoryPriority::Low
        );
        assert_eq!(
            InventoryPriority::parse("medium").expect("Priority should be valid"),
            InventoryPriority::Medium
        );
        assert_eq!(
            InventoryPriority::parse("high").expect("Priority should be valid"),
            InventoryPriority::High
        );
        assert_eq!(
            InventoryPriority::parse("default").expect("Priority should be valid"),
            InventoryPriority::Default
        )
    }

    #[test]
    fn invalid_string_is_rejected() {
        assert_eq!(
            InventoryPriority::parse("invalid"),
            Err(InventoryPriorityError::Invalid)
        )
    }

    #[test]
    fn as_str_returns_correct_value() {
        assert_eq!(InventoryPriority::Low.as_str(), "low");
        assert_eq!(InventoryPriority::Medium.as_str(), "medium");
        assert_eq!(InventoryPriority::High.as_str(), "high");
        assert_eq!(InventoryPriority::Default.as_str(), "default")
    }

    #[test]
    fn default_is_inventory_priority_default() {
        assert_eq!(InventoryPriority::default(), InventoryPriority::Default)
    }
}
