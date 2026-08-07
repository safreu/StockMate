use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::modules::{
    accounts::domain::UserId,
    households::{
        adapters::validate::validate_aggregate,
        domain::{Household, HouseholdId, HouseholdKind, HouseholdMember},
        ports::{HouseholdRepository, HouseholdRepositoryError},
    },
};

struct HouseholdState {
    households: HashMap<HouseholdId, Household>,
    members: HashMap<(HouseholdId, UserId), HouseholdMember>,
}

pub struct InMemoryHouseholdRepository {
    state: RwLock<HouseholdState>,
}

impl InMemoryHouseholdRepository {
    pub fn new() -> Self {
        let households: HashMap<HouseholdId, Household> = HashMap::new();
        let members: HashMap<(HouseholdId, UserId), HouseholdMember> = HashMap::new();

        let state = HouseholdState {
            households,
            members,
        };
        Self {
            state: RwLock::new(state),
        }
    }
}

impl Default for InMemoryHouseholdRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HouseholdRepository for InMemoryHouseholdRepository {
    async fn create_with_owner(
        &self,
        household: &Household,
        owner: &HouseholdMember,
    ) -> Result<(), HouseholdRepositoryError> {
        let mut state = self.state.write().await;

        validate_aggregate(household, owner)?;

        if household.kind() == HouseholdKind::Personal {
            let already_exists = state.households.values().any(|existing| {
                existing.kind() == HouseholdKind::Personal
                    && existing.personal_owner_id() == Some(owner.user_id())
            });
            if already_exists {
                return Err(HouseholdRepositoryError::PersonalHouseholdAlreadyExists);
            }
        }

        if state.households.contains_key(&household.id()) {
            return Err(HouseholdRepositoryError::HouseholdAlreadyExists);
        }

        state.households.insert(household.id(), household.clone());

        state
            .members
            .insert((owner.household_id(), owner.user_id()), owner.clone());

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &HouseholdId,
    ) -> Result<Option<Household>, HouseholdRepositoryError> {
        let state = self.state.read().await;
        Ok(state.households.get(id).cloned())
    }

    async fn find_personal_by_owner(
        &self,
        owner: &UserId,
    ) -> Result<Option<Household>, HouseholdRepositoryError> {
        let state = self.state.read().await;

        let household = state
            .households
            .values()
            .find(|household| {
                household.kind() == HouseholdKind::Personal
                    && household.personal_owner_id() == Some(*owner)
            })
            .cloned();

        Ok(household)
    }

    async fn find_for_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Household>, HouseholdRepositoryError> {
        let state = self.state.read().await;

        let households = state
            .members
            .values()
            .filter(|member| member.user_id() == *user_id)
            .filter_map(|member| state.households.get(&member.household_id()).cloned())
            .collect();

        Ok(households)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::modules::households::domain::{HouseholdError, HouseholdName, HouseholdRole};

    use super::*;

    fn create_household(
        id: HouseholdId,
        kind: HouseholdKind,
        personal_owner_id: Option<UserId>,
    ) -> Result<Household, HouseholdError> {
        Household::new(
            id,
            HouseholdName::parse("This is a name").expect("Test name should be valid"),
            kind,
            personal_owner_id,
            Utc::now(),
            Utc::now(),
        )
    }

    fn create_owned_household(
        user_id: UserId,
        kind: HouseholdKind,
    ) -> (Household, HouseholdMember) {
        let household_id = HouseholdId::new();
        let now = Utc::now();

        let owner = HouseholdMember::new(household_id, user_id, HouseholdRole::Owner, now);

        let personal_owner_id = match kind {
            HouseholdKind::Personal => Some(user_id),
            HouseholdKind::Shared => None,
        };

        let household = Household::new(
            household_id,
            HouseholdName::parse("Test household").expect("Test household name should be valid"),
            kind,
            personal_owner_id,
            now,
            now,
        )
        .expect("Test household should be valid");

        (household, owner)
    }

    fn create_household_member(
        household_id: HouseholdId,
        user_id: UserId,
        role: HouseholdRole,
    ) -> HouseholdMember {
        HouseholdMember::new(household_id, user_id, role, Utc::now())
    }

    #[tokio::test]
    async fn personal_household_with_owner_can_be_created() {
        let repository = InMemoryHouseholdRepository::new();

        let (household, owner) = create_owned_household(UserId::new(), HouseholdKind::Personal);

        let result = repository.create_with_owner(&household, &owner).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn shared_household_with_owner_can_be_created() {
        let repository = InMemoryHouseholdRepository::new();

        let (household, owner) = create_owned_household(UserId::new(), HouseholdKind::Shared);

        let result = repository.create_with_owner(&household, &owner).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn created_household_can_be_found_by_id() {
        let repository = InMemoryHouseholdRepository::new();

        let (household, owner) = create_owned_household(UserId::new(), HouseholdKind::Shared);

        repository
            .create_with_owner(&household, &owner)
            .await
            .expect("Household creation should succeed");

        let result = repository
            .find_by_id(&household.id())
            .await
            .expect("Household lookup should succeed");

        assert_eq!(result, Some(household))
    }

    #[tokio::test]
    async fn personal_household_can_be_found_by_owner() {
        let repository = InMemoryHouseholdRepository::new();

        let (household, owner) = create_owned_household(UserId::new(), HouseholdKind::Personal);

        repository
            .create_with_owner(&household, &owner)
            .await
            .expect("Household creation should succeed");

        let result = repository
            .find_personal_by_owner(&owner.user_id())
            .await
            .expect("Household lookup should succeed");

        assert_eq!(result, Some(household))
    }

    #[tokio::test]
    async fn households_for_user_are_returned() {
        let repository = InMemoryHouseholdRepository::new();

        let user_id = UserId::new();

        let (personal_household, personal_owner) =
            create_owned_household(user_id, HouseholdKind::Personal);
        let (shared_household, shared_owner) =
            create_owned_household(user_id, HouseholdKind::Shared);

        repository
            .create_with_owner(&personal_household, &personal_owner)
            .await
            .expect("Personal household creation should succeed");
        repository
            .create_with_owner(&shared_household, &shared_owner)
            .await
            .expect("Shared household creation should succeed");

        let result = repository
            .find_for_user(&personal_owner.user_id())
            .await
            .expect("Household lookup should succeed");

        assert_eq!(result.len(), 2);
        assert!(result.contains(&personal_household));
        assert!(result.contains(&shared_household))
    }

    #[tokio::test]
    async fn duplicate_personal_household_is_rejected() {
        let repository = InMemoryHouseholdRepository::new();

        let user_id = UserId::new();

        let (personal_household, first_owner) =
            create_owned_household(user_id, HouseholdKind::Personal);
        let (another_personal_household, second_owner) =
            create_owned_household(user_id, HouseholdKind::Personal);

        repository
            .create_with_owner(&personal_household, &first_owner)
            .await
            .expect("Household creation should succeed");

        let result = repository
            .create_with_owner(&another_personal_household, &second_owner)
            .await;

        assert_eq!(
            result,
            Err(HouseholdRepositoryError::PersonalHouseholdAlreadyExists)
        )
    }

    #[tokio::test]
    async fn inconsistent_owner_membership_is_rejected() {
        let repository = InMemoryHouseholdRepository::new();

        let user_id = UserId::new();
        let household_id = HouseholdId::new();
        let another_household_id = HouseholdId::new();

        let household = create_household(household_id, HouseholdKind::Shared, None)
            .expect("Test household should be valid");

        let owner = create_household_member(another_household_id, user_id, HouseholdRole::Owner);

        let result = repository.create_with_owner(&household, &owner).await;

        assert_eq!(result, Err(HouseholdRepositoryError::InvalidAggregate))
    }

    #[tokio::test]
    async fn non_owner_membership_is_rejected() {
        let repository = InMemoryHouseholdRepository::new();

        let user_id = UserId::new();
        let household_id = HouseholdId::new();

        let household = create_household(household_id, HouseholdKind::Shared, None)
            .expect("Test household should be valid");

        let owner = create_household_member(household_id, user_id, HouseholdRole::Member);

        let result = repository.create_with_owner(&household, &owner).await;

        assert_eq!(result, Err(HouseholdRepositoryError::InvalidAggregate))
    }

    #[tokio::test]
    async fn unknown_household_returns_none() {
        let repository = InMemoryHouseholdRepository::default();

        let result = repository
            .find_by_id(&HouseholdId::new())
            .await
            .expect("Household lookup should succeed");

        assert!(result.is_none())
    }

    #[tokio::test]
    async fn user_without_households_returns_empty_list() {
        let repository = InMemoryHouseholdRepository::default();

        let result = repository
            .find_for_user(&UserId::new())
            .await
            .expect("Household lookup should succeed");

        assert!(result.is_empty())
    }
}
