use std::sync::Arc;

use crate::{
    modules::{
        accounts::domain::UserId,
        households::{
            domain::{Household, HouseholdId},
            ports::HouseholdRepository,
        },
    },
    shared::application::InternalError,
};

pub struct GetHouseholdCommand {
    pub household_id: HouseholdId,
    pub user_id: UserId,
}

pub struct GetHouseholdService {
    household_repository: Arc<dyn HouseholdRepository>,
}

impl GetHouseholdService {
    pub fn new(household_repository: Arc<dyn HouseholdRepository>) -> Self {
        Self {
            household_repository,
        }
    }

    pub async fn execute(
        &self,
        command: GetHouseholdCommand,
    ) -> Result<Household, GetHouseholdError> {
        let household = self
            .household_repository
            .find_by_id(&command.household_id)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    "failed to load household"
                );
                InternalError::Failed
            })?
            .ok_or(GetHouseholdError::NotFound)?;

        let member = self
            .household_repository
            .find_member(&command.household_id, &command.user_id)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    "failed to load household membership",
                );
                InternalError::Failed
            })?;

        if member.is_none() {
            return Err(GetHouseholdError::Forbidden);
        }

        Ok(household)
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum GetHouseholdError {
    #[error("Household not found")]
    NotFound,
    #[error("User is not a member of this household")]
    Forbidden,
    #[error(transparent)]
    Internal(#[from] InternalError),
}

#[cfg(test)]
mod tests {
    use crate::{
        modules::households::domain::HouseholdKind,
        test_helpers::{
            FailingHouseholdRepository, build_get_household_service, create_owned_household,
            insert_owned_household,
        },
    };

    use super::*;

    #[tokio::test]
    async fn member_can_retrieve_household() {
        let (service, repository) = build_get_household_service();

        let user_id = UserId::new();

        let (household, _) =
            insert_owned_household(&repository, user_id, HouseholdKind::Shared).await;

        let result = service
            .execute(GetHouseholdCommand {
                household_id: household.id(),
                user_id,
            })
            .await
            .expect("Household lookup should succeed");

        assert_eq!(result, household)
    }

    #[tokio::test]
    async fn unknown_household_returns_not_found() {
        let (service, _) = build_get_household_service();

        let result = service
            .execute(GetHouseholdCommand {
                household_id: HouseholdId::new(),
                user_id: UserId::new(),
            })
            .await;

        assert_eq!(result, Err(GetHouseholdError::NotFound))
    }

    #[tokio::test]
    async fn non_member_returns_forbidden() {
        let (service, repository) = build_get_household_service();

        let user_id = UserId::new();

        let (household, owner) = create_owned_household(user_id, HouseholdKind::Shared);

        repository
            .create_with_owner(&household, &owner)
            .await
            .expect("Household creation should succeed");

        let result = service
            .execute(GetHouseholdCommand {
                household_id: household.id(),
                user_id: UserId::new(),
            })
            .await;

        assert_eq!(result, Err(GetHouseholdError::Forbidden))
    }

    #[tokio::test]
    async fn repository_failure_returns_internal_error() {
        let repository = Arc::new(FailingHouseholdRepository);
        let service = GetHouseholdService::new(repository);

        let result = service
            .execute(GetHouseholdCommand {
                household_id: HouseholdId::new(),
                user_id: UserId::new(),
            })
            .await;

        assert_eq!(
            result,
            Err(GetHouseholdError::Internal(InternalError::Failed))
        )
    }
}
