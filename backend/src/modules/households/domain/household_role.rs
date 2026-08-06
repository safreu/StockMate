use core::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseholdRole {
    Owner,
    Member,
}

impl HouseholdRole {
    pub fn parse(value: &str) -> Result<Self, HouseholdRoleError> {
        value.parse()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Member => "member",
        }
    }
}

impl fmt::Display for HouseholdRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for HouseholdRole {
    type Err = HouseholdRoleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "owner" => Ok(Self::Owner),
            "member" => Ok(Self::Member),
            _ => Err(HouseholdRoleError::Invalid),
        }
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum HouseholdRoleError {
    #[error("The given household role is invalid")]
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_can_be_parsed() {
        assert_eq!(HouseholdRole::parse("owner"), Ok(HouseholdRole::Owner))
    }

    #[test]
    fn member_can_be_parsed() {
        assert_eq!(HouseholdRole::parse("member"), Ok(HouseholdRole::Member))
    }

    #[test]
    fn unknown_role_is_rejected() {
        assert_eq!(
            HouseholdRole::parse("unknown"),
            Err(HouseholdRoleError::Invalid)
        )
    }

    #[test]
    fn as_str_returns_database_representation() {
        assert_eq!(HouseholdRole::Owner.as_str(), "owner");
        assert_eq!(HouseholdRole::Member.as_str(), "member")
    }
}
