use chrono::Utc;

use crate::modules::accounts::adapters::InMemoryUserRepository;
use crate::modules::accounts::domain::UserId;
use crate::modules::households::domain::{
    HouseholdId, HouseholdKind, HouseholdName, HouseholdRole,
};
use crate::modules::households::ports::HouseholdRepository;
use crate::modules::households::{
    adapters::InMemoryHouseholdRepository,
    domain::{Household, HouseholdMember},
};
use crate::test_helpers::{SharedHouseholdFixture, insert_user};

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

pub async fn insert_owned_household(
    repository: &InMemoryHouseholdRepository,
    owner_id: UserId,
    kind: HouseholdKind,
) -> (Household, HouseholdMember) {
    let (household, owner) = create_owned_household(owner_id, kind);

    repository
        .create_with_owner(&household, &owner)
        .await
        .expect("Test household should be insertable");

    (household, owner)
}

pub async fn insert_member(
    repository: &InMemoryHouseholdRepository,
    household_id: HouseholdId,
    user_id: UserId,
) -> HouseholdMember {
    let member = HouseholdMember::new(household_id, user_id, HouseholdRole::Member, Utc::now());

    repository
        .add_member(&member)
        .await
        .expect("Test member should be insertable");

    member
}

pub async fn create_shared_household_fixture(
    user_repository: &InMemoryUserRepository,
    household_repository: &InMemoryHouseholdRepository,
) -> SharedHouseholdFixture {
    let owner = insert_user(user_repository, "owner@email.com").await;

    let (household, _) =
        insert_owned_household(household_repository, owner.id(), HouseholdKind::Shared).await;

    SharedHouseholdFixture { owner, household }
}
