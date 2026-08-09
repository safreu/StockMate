use std::sync::Arc;

use crate::{
    modules::{
        accounts::domain::UserId,
        households::{
            domain::{HouseholdId, HouseholdRole},
            ports::{HouseholdRepository, HouseholdRepositoryError},
        },
    },
    shared::application::InternalError,
};

pub struct RemoveHouseholdMemberCommand {
    pub requester_id: UserId,
    pub household_id: HouseholdId,
    pub member_id: UserId,
}

pub struct RemoveHouseholdMemberService {
    household_repository: Arc<dyn HouseholdRepository>,
}

impl RemoveHouseholdMemberService {
    pub fn new(household_repository: Arc<dyn HouseholdRepository>) -> Self {
        Self {
            household_repository,
        }
    }

    pub async fn execute(
        &self,
        command: RemoveHouseholdMemberCommand,
    ) -> Result<(), RemoveHouseholdMemberError> {
        self.household_repository
            .find_by_id(&command.household_id)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    household_id = %command.household_id,
                    "Failed to load household",
                );
                InternalError::Failed
            })?
            .ok_or(RemoveHouseholdMemberError::HouseholdNotFound)?;

        let requester = self
            .household_repository
            .find_member(&command.household_id, &command.requester_id)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    household_id = %command.household_id,
                    requester_id = %command.requester_id,
                    "Failed to load requester membership"
                );
                InternalError::Failed
            })?
            .ok_or(RemoveHouseholdMemberError::Forbidden)?;

        if !(command.requester_id == command.member_id) && requester.role() != HouseholdRole::Owner
        {
            return Err(RemoveHouseholdMemberError::Forbidden);
        }

        let target = self
            .household_repository
            .find_member(&command.household_id, &command.member_id)
            .await
            .map_err(|error| {
                tracing::error!(
                    error = ?error,
                    household_id = %command.household_id,
                    member_id = %command.member_id,
                    "Failed to load target membership",
                );
                InternalError::Failed
            })?
            .ok_or(RemoveHouseholdMemberError::MemberNotFound)?;

        if target.role() == HouseholdRole::Owner {
            return Err(RemoveHouseholdMemberError::OwnerCannotBeRemoved);
        }

        self.household_repository
            .remove_member(&command.household_id, &command.member_id)
            .await
            .map_err(|error| match error {
                HouseholdRepositoryError::MemberNotFound => {
                    RemoveHouseholdMemberError::MemberNotFound
                }
                other => {
                    tracing::error!(
                        error = ?other,
                        household_id = %command.household_id,
                        member_id = %command.member_id,
                        "Failed to remove household member"
                    );
                    RemoveHouseholdMemberError::Internal(InternalError::Failed)
                }
            })?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum RemoveHouseholdMemberError {
    #[error("Household was not found")]
    HouseholdNotFound,
    #[error("You do not have permissions")]
    Forbidden,
    #[error("Household member not found")]
    MemberNotFound,
    #[error("The household owner cannot be removed")]
    OwnerCannotBeRemoved,
    #[error(transparent)]
    Internal(#[from] InternalError),
}
