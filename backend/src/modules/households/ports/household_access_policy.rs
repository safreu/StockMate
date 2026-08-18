use async_trait::async_trait;

use crate::{
    modules::{
        accounts::domain::UserId,
        households::domain::{HouseholdId, HouseholdMember},
    },
    shared::application::InternalError,
};

#[async_trait]
pub trait HouseholdAccessPolicy: Send + Sync {
    async fn require_member(
        &self,
        household_id: &HouseholdId,
        user_id: &UserId,
    ) -> Result<HouseholdMember, HouseholdAccessError>;

    async fn require_owner(
        &self,
        household_id: &HouseholdId,
        user_id: &UserId,
    ) -> Result<HouseholdMember, HouseholdAccessError>;
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum HouseholdAccessError {
    #[error("Household was not found")]
    HouseholdNotFound,
    #[error("You do not have permission")]
    Forbidden,
    #[error(transparent)]
    Internal(#[from] InternalError),
}
