use chrono::{DateTime, Utc};

use crate::modules::{
    accounts::domain::UserId,
    households::domain::{HouseholdId, HouseholdRole},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseholdMember {
    household_id: HouseholdId,
    user_id: UserId,
    role: HouseholdRole,
    created_at: DateTime<Utc>,
}

impl HouseholdMember {
    pub fn new(
        household_id: HouseholdId,
        user_id: UserId,
        role: HouseholdRole,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            household_id,
            user_id,
            role,
            created_at,
        }
    }

    pub fn household_id(&self) -> HouseholdId {
        self.household_id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn role(&self) -> HouseholdRole {
        self.role
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_is_created_with_given_values() {
        let household_id = HouseholdId::new();
        let user_id = UserId::new();
        let created_at = Utc::now();

        let member = HouseholdMember::new(household_id, user_id, HouseholdRole::Owner, created_at);

        assert_eq!(member.household_id(), household_id);
        assert_eq!(member.user_id(), user_id);
        assert_eq!(member.role(), HouseholdRole::Owner);
        assert_eq!(member.created_at(), created_at)
    }
}
