use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;

use crate::modules::accounts::adapters::InMemoryUserRepository;
use crate::modules::accounts::domain::UserId;
use crate::modules::households::application::{
    AddHouseholdMemberService, CreateHouseholdService, GetHouseholdService,
    ListHouseholdMembersService, ListHouseholdsForUserService,
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

    async fn add_member(&self, member: &HouseholdMember) -> Result<(), HouseholdRepositoryError> {
        Err(HouseholdRepositoryError::Persistence(
            PersistenceError::Failed,
        ))
    }

    async fn find_members(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<HouseholdMember>, HouseholdRepositoryError> {
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

pub fn build_add_member_service() -> (
    AddHouseholdMemberService,
    Arc<InMemoryHouseholdRepository>,
    Arc<InMemoryUserRepository>,
) {
    let household_repository = Arc::new(InMemoryHouseholdRepository::new());
    let user_repository = Arc::new(InMemoryUserRepository::new());

    let service =
        AddHouseholdMemberService::new(household_repository.clone(), user_repository.clone());

    (service, household_repository, user_repository)
}

pub struct DuplicateOnAddHouseholdRepository {
    pub inner: Arc<InMemoryHouseholdRepository>,
}

#[async_trait::async_trait]
impl HouseholdRepository for DuplicateOnAddHouseholdRepository {
    async fn create_with_owner(
        &self,
        household: &Household,
        owner: &HouseholdMember,
    ) -> Result<(), HouseholdRepositoryError> {
        self.inner.create_with_owner(household, owner).await
    }

    async fn find_by_id(
        &self,
        id: &HouseholdId,
    ) -> Result<Option<Household>, HouseholdRepositoryError> {
        self.inner.find_by_id(id).await
    }

    async fn find_personal_by_owner(
        &self,
        owner: &UserId,
    ) -> Result<Option<Household>, HouseholdRepositoryError> {
        self.inner.find_personal_by_owner(owner).await
    }

    async fn find_for_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Household>, HouseholdRepositoryError> {
        self.inner.find_for_user(user_id).await
    }

    async fn find_member(
        &self,
        household_id: &HouseholdId,
        user_id: &UserId,
    ) -> Result<Option<HouseholdMember>, HouseholdRepositoryError> {
        self.inner.find_member(household_id, user_id).await
    }

    #[allow(unused_variables)]
    async fn add_member(&self, member: &HouseholdMember) -> Result<(), HouseholdRepositoryError> {
        Err(HouseholdRepositoryError::MemberAlreadyExists)
    }

    async fn find_members(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<HouseholdMember>, HouseholdRepositoryError> {
        self.inner.find_members(household_id).await
    }
}

pub fn build_list_household_members_service() -> (
    ListHouseholdMembersService,
    Arc<InMemoryHouseholdRepository>,
    Arc<InMemoryUserRepository>,
) {
    let household_repository = Arc::new(InMemoryHouseholdRepository::new());
    let user_repository = Arc::new(InMemoryUserRepository::new());

    let service =
        ListHouseholdMembersService::new(household_repository.clone(), user_repository.clone());

    (service, household_repository, user_repository)
}
