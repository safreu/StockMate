use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;

use crate::modules::accounts::domain::UserId;
use crate::modules::households::application::{
    CreateHouseholdService, GetHouseholdService, ListHouseholdsForUserService,
};
use crate::modules::households::domain::{
    HouseholdId, HouseholdKind, HouseholdName, HouseholdRole,
};
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

pub fn build_get_household_service() -> (GetHouseholdService, Arc<InMemoryHouseholdRepository>) {
    let repository = Arc::new(InMemoryHouseholdRepository::new());

    let service = GetHouseholdService::new(repository.clone());

    (service, repository)
}

pub fn create_owned_household(
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
