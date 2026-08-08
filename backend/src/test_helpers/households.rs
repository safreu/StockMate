use std::sync::Arc;

use async_trait::async_trait;

use crate::modules::accounts::domain::UserId;
use crate::modules::households::application::{
    CreateHouseholdService, ListHouseholdsForUserService,
};
use crate::modules::households::domain::HouseholdId;
use crate::modules::households::ports::HouseholdRepository;
use crate::{
    modules::households::{
        adapters::InMemoryHouseholdRepository,
        domain::{Household, HouseholdMember},
        ports::HouseholdRepositoryError,
    },
    shared::db::PersistenceError,
};

pub struct FailingHouseholdRepository;

#[async_trait]
#[allow(unused)]
impl HouseholdRepository for FailingHouseholdRepository {
    async fn create_with_owner(
        &self,
        household: &Household,
        owner: &HouseholdMember,
    ) -> Result<(), HouseholdRepositoryError> {
        Err(HouseholdRepositoryError::Persistence(
            PersistenceError::Failed,
        ))
    }

    async fn find_by_id(
        &self,
        id: &HouseholdId,
    ) -> Result<Option<Household>, HouseholdRepositoryError> {
        Err(HouseholdRepositoryError::Persistence(
            PersistenceError::Failed,
        ))
    }

    async fn find_personal_by_owner(
        &self,
        owner: &UserId,
    ) -> Result<Option<Household>, HouseholdRepositoryError> {
        Err(HouseholdRepositoryError::Persistence(
            PersistenceError::Failed,
        ))
    }

    async fn find_for_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Household>, HouseholdRepositoryError> {
        Err(HouseholdRepositoryError::Persistence(
            PersistenceError::Failed,
        ))
    }

    async fn find_member(
        &self,
        household_id: &HouseholdId,
        user_id: &UserId,
    ) -> Result<Option<HouseholdMember>, HouseholdRepositoryError> {
        Err(HouseholdRepositoryError::Persistence(
            PersistenceError::Failed,
        ))
    }
}

pub fn build_create_household_service() -> (CreateHouseholdService, Arc<InMemoryHouseholdRepository>)
{
    let repository = Arc::new(InMemoryHouseholdRepository::new());

    let service = CreateHouseholdService::new(repository.clone());

    (service, repository)
}

pub fn build_list_households_service() -> (
    ListHouseholdsForUserService,
    Arc<InMemoryHouseholdRepository>,
) {
    let repository = Arc::new(InMemoryHouseholdRepository::new());

    let service = ListHouseholdsForUserService::new(repository.clone());

    (service, repository)
}
