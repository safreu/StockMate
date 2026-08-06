use chrono::{DateTime, Utc};

use crate::modules::{
    accounts::domain::UserId,
    households::domain::{HouseholdId, HouseholdKind, HouseholdName},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Household {
    id: HouseholdId,
    name: HouseholdName,
    kind: HouseholdKind,
    personal_owner_id: Option<UserId>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Household {
    pub fn new(
        id: HouseholdId,
        name: HouseholdName,
        kind: HouseholdKind,
        personal_owner_id: Option<UserId>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, HouseholdError> {
        if updated_at < created_at {
            return Err(HouseholdError::InvalidTimestamps);
        }

        match (kind, personal_owner_id) {
            (HouseholdKind::Personal, None) => {
                return Err(HouseholdError::PersonalHouseholdRequiresOwner);
            }
            (HouseholdKind::Shared, Some(_)) => {
                return Err(HouseholdError::SharedHouseholdCannotHavePersonalOwner);
            }
            _ => {}
        }

        Ok(Self {
            id,
            name,
            kind,
            personal_owner_id,
            created_at,
            updated_at,
        })
    }

    pub fn id(&self) -> HouseholdId {
        self.id
    }

    pub fn name(&self) -> &HouseholdName {
        &self.name
    }

    pub fn kind(&self) -> HouseholdKind {
        self.kind
    }

    pub fn personal_owner_id(&self) -> Option<UserId> {
        self.personal_owner_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum HouseholdError {
    #[error("Updated time cannot be before creation time")]
    InvalidTimestamps,
    #[error("Personal household requires an owner")]
    PersonalHouseholdRequiresOwner,
    #[error("Shared household cannot have a personal owner")]
    SharedHouseholdCannotHavePersonalOwner,
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;

    fn test_household(
        kind: HouseholdKind,
        personal_owner_id: Option<UserId>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Household, HouseholdError> {
        Household::new(
            HouseholdId::new(),
            HouseholdName::parse("valid household name")
                .expect("Test household name should be valid"),
            kind,
            personal_owner_id,
            created_at,
            updated_at,
        )
    }

    #[test]
    fn valid_personal_household_is_accepted() {
        let user_id = UserId::new();

        let now = Utc::now();

        let household = test_household(HouseholdKind::Personal, Some(user_id), now, now);

        assert!(household.is_ok())
    }

    #[test]
    fn personal_household_without_owner_is_rejected() {
        let now = Utc::now();

        let household = test_household(HouseholdKind::Personal, None, now, now);

        assert_eq!(
            household,
            Err(HouseholdError::PersonalHouseholdRequiresOwner)
        )
    }

    #[test]
    fn valid_shared_household_is_accepted() {
        let now = Utc::now();

        let household = test_household(HouseholdKind::Shared, None, now, now);

        assert!(household.is_ok())
    }

    #[test]
    fn shared_household_with_personal_owner_is_rejected() {
        let user_id = UserId::new();

        let now = Utc::now();

        let household = test_household(HouseholdKind::Shared, Some(user_id), now, now);

        assert_eq!(
            household,
            Err(HouseholdError::SharedHouseholdCannotHavePersonalOwner)
        )
    }

    #[test]
    fn equal_timestamps_are_accepted() {
        let now = Utc::now();

        let household = test_household(HouseholdKind::Shared, None, now, now);

        assert!(household.is_ok())
    }

    #[test]
    fn updated_at_before_created_at_is_rejected() {
        let created_at = Utc::now();
        let updated_at = created_at - Duration::hours(1);

        let household = test_household(HouseholdKind::Shared, None, created_at, updated_at);

        assert_eq!(household, Err(HouseholdError::InvalidTimestamps))
    }
}
