use std::sync::Arc;

use async_trait::async_trait;

use crate::modules::accounts::domain::UserId;
use crate::modules::households::{
    domain::{HouseholdId, HouseholdMember, HouseholdRole},
    ports::{HouseholdAccessError, HouseholdAccessPolicy, HouseholdRepository},
};
use crate::shared::application::InternalError;
pub struct DefaultHouseholdAccessPolicy {
    household_repository: Arc<dyn HouseholdRepository>,
}

impl DefaultHouseholdAccessPolicy {
    pub fn new(household_repository: Arc<dyn HouseholdRepository>) -> Self {
        Self {
            household_repository,
        }
    }
}

#[async_trait]
impl HouseholdAccessPolicy for DefaultHouseholdAccessPolicy {
    async fn require_member(
        &self,
        household_id: &HouseholdId,
        user_id: &UserId,
    ) -> Result<HouseholdMember, HouseholdAccessError> {
        self.household_repository
            .find_by_id(household_id)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    household_id = %household_id,
                    "Failed to load household"
                );
                InternalError::Failed
            })?
            .ok_or(HouseholdAccessError::HouseholdNotFound)?;

        self.household_repository
            .find_member(household_id, user_id)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    household_id = %household_id,
                    user_id = %user_id,
                    "Failed to load household membership",
                );
                InternalError::Failed
            })?
            .ok_or(HouseholdAccessError::Forbidden)
    }

    async fn require_owner(
        &self,
        household_id: &HouseholdId,
        user_id: &UserId,
    ) -> Result<HouseholdMember, HouseholdAccessError> {
        let member = self.require_member(household_id, user_id).await?;

        if member.role() != HouseholdRole::Owner {
            return Err(HouseholdAccessError::Forbidden);
        }
        Ok(member)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::{
        modules::households::{adapters::InMemoryHouseholdRepository, domain::HouseholdKind},
        test_helpers::create_owned_household,
    };

    use super::*;

    #[tokio::test]
    async fn household_member_is_allowed() {
        let household_repository = Arc::new(InMemoryHouseholdRepository::default());
        let policy = DefaultHouseholdAccessPolicy::new(household_repository.clone());

        let user_id = UserId::new();

        let (household, owner_membership) = create_owned_household(user_id, HouseholdKind::Shared);

        household_repository
            .create_with_owner(&household, &owner_membership)
            .await
            .expect("Household creation should succeed");

        let member = policy
            .require_member(&household.id(), &user_id)
            .await
            .expect("Household member should be allowed");

        assert_eq!(member.user_id(), user_id);
        assert_eq!(member.household_id(), household.id());
    }

    #[tokio::test]
    async fn non_member_is_forbidden() {
        let household_repository = Arc::new(InMemoryHouseholdRepository::default());
        let policy = DefaultHouseholdAccessPolicy::new(household_repository.clone());

        let user_id = UserId::new();

        let (household, owner_membership) = create_owned_household(user_id, HouseholdKind::Shared);

        household_repository
            .create_with_owner(&household, &owner_membership)
            .await
            .expect("Household creation should succeed");

        let result = policy.require_member(&household.id(), &UserId::new()).await;

        assert_eq!(result, Err(HouseholdAccessError::Forbidden))
    }

    #[tokio::test]
    async fn unknown_household_returns_household_not_found() {
        let household_repository = Arc::new(InMemoryHouseholdRepository::default());
        let policy = DefaultHouseholdAccessPolicy::new(household_repository.clone());

        let user_id = UserId::new();

        let (household, owner_membership) = create_owned_household(user_id, HouseholdKind::Shared);

        household_repository
            .create_with_owner(&household, &owner_membership)
            .await
            .expect("Household creation should succeed");

        let result = policy.require_member(&HouseholdId::new(), &user_id).await;

        assert_eq!(result, Err(HouseholdAccessError::HouseholdNotFound))
    }

    #[tokio::test]
    async fn household_owner_is_allowed_as_owner() {
        let household_repository = Arc::new(InMemoryHouseholdRepository::default());
        let policy = DefaultHouseholdAccessPolicy::new(household_repository.clone());

        let user_id = UserId::new();

        let (household, owner_membership) = create_owned_household(user_id, HouseholdKind::Shared);

        household_repository
            .create_with_owner(&household, &owner_membership)
            .await
            .expect("Household creation should succeed");

        let member = policy
            .require_owner(&household.id(), &user_id)
            .await
            .expect("Household owner should be allowed");

        assert_eq!(member.user_id(), user_id);
        assert_eq!(member.household_id(), household.id());
    }

    #[tokio::test]
    async fn normal_member_is_forbidden_as_owner() {
        let household_repository = Arc::new(InMemoryHouseholdRepository::default());
        let policy = DefaultHouseholdAccessPolicy::new(household_repository.clone());

        let user_id = UserId::new();

        let (household, owner_membership) = create_owned_household(user_id, HouseholdKind::Shared);

        household_repository
            .create_with_owner(&household, &owner_membership)
            .await
            .expect("Household creation should succeed");

        let member = &HouseholdMember::new(
            household.id(),
            UserId::new(),
            HouseholdRole::Member,
            Utc::now(),
        );

        household_repository
            .add_member(member)
            .await
            .expect("Household member adding should succeed");

        let result = policy
            .require_owner(&household.id(), &member.user_id())
            .await;

        assert_eq!(result, Err(HouseholdAccessError::Forbidden))
    }
}
