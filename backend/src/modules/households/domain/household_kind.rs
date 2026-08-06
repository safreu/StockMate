use core::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseholdKind {
    Personal,
    Shared,
}

impl HouseholdKind {
    pub fn parse(value: &str) -> Result<Self, HouseholdKindError> {
        value.parse()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Personal => "personal",
            Self::Shared => "shared",
        }
    }
}

impl fmt::Display for HouseholdKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for HouseholdKind {
    type Err = HouseholdKindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "personal" => Ok(Self::Personal),
            "shared" => Ok(Self::Shared),
            _ => Err(HouseholdKindError::Invalid),
        }
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum HouseholdKindError {
    #[error("The given household kind is invalid")]
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn personal_can_be_parsed() {
        assert_eq!(
            HouseholdKind::parse("personal"),
            Ok(HouseholdKind::Personal)
        )
    }

    #[test]
    fn shared_can_be_parsed() {
        assert_eq!(HouseholdKind::parse("shared"), Ok(HouseholdKind::Shared))
    }

    #[test]
    fn unknown_kind_is_rejected() {
        assert_eq!(
            HouseholdKind::parse("unknown"),
            Err(HouseholdKindError::Invalid)
        )
    }

    #[test]
    fn as_str_returns_database_representation() {
        assert_eq!(HouseholdKind::Personal.as_str(), "personal");
        assert_eq!(HouseholdKind::Shared.as_str(), "shared")
    }
}
