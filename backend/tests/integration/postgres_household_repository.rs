use backend::modules::{
    accounts::{
        adapters::PostgresUserRepository,
        domain::{User, UserId},
        ports::UserRepository,
    },
    households::{
        adapters::PostgresHouseholdRepository,
        domain::{
            Household, HouseholdId, HouseholdKind, HouseholdMember, HouseholdName, HouseholdRole,
        },
        ports::{HouseholdRepository, HouseholdRepositoryError},
    },
};
use chrono::{SubsecRound, Utc};
use sqlx::PgPool;

use crate::integration::helpers::test_user;

async fn insert_test_user(repository: &PostgresUserRepository) -> User {
    let user = test_user("valid@email.com");

    repository
        .insert(&user)
        .await
        .expect("Test user should be insertable");

    user
}

fn create_owned_household(user_id: UserId, kind: HouseholdKind) -> (Household, HouseholdMember) {
    let household_id = HouseholdId::new();
    let now = Utc::now().trunc_subsecs(6);

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

    let owner = HouseholdMember::new(household_id, user_id, HouseholdRole::Owner, now);

    (household, owner)
}

async fn insert_owned_household(
    repository: &PostgresHouseholdRepository,
    user_id: UserId,
    kind: HouseholdKind,
) -> (Household, HouseholdMember) {
    let (household, owner) = create_owned_household(user_id, kind);

    repository
        .create_with_owner(&household, &owner)
        .await
        .expect("Test household should be insertable");

    (household, owner)
}

#[sqlx::test]
async fn personal_household_with_owner_can_be_created(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let user = insert_test_user(&user_repository).await;

    let (household, owner) = create_owned_household(user.id(), HouseholdKind::Personal);

    let result = household_repository
        .create_with_owner(&household, &owner)
        .await;

    assert!(result.is_ok())
}

#[sqlx::test]
async fn shared_household_with_owner_can_be_created(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let user = insert_test_user(&user_repository).await;

    let (household, owner) = create_owned_household(user.id(), HouseholdKind::Shared);

    let result = household_repository
        .create_with_owner(&household, &owner)
        .await;

    assert!(result.is_ok())
}

#[sqlx::test]
async fn created_household_can_be_found_by_id(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let user = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, user.id(), HouseholdKind::Shared).await;

    let result = household_repository
        .find_by_id(&household.id())
        .await
        .expect("Household repository lookup should succeed");

    assert_eq!(result, Some(household))
}

#[sqlx::test]
async fn personal_household_can_be_found_by_owner(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let user = insert_test_user(&user_repository).await;

    let (household, owner) =
        insert_owned_household(&household_repository, user.id(), HouseholdKind::Personal).await;

    let result = household_repository
        .find_personal_by_owner(&owner.user_id())
        .await
        .expect("Household repository lookup should succeed");

    assert_eq!(result, Some(household))
}

#[sqlx::test]
async fn households_for_user_are_returned(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let user = insert_test_user(&user_repository).await;

    let (personal_household, _) =
        insert_owned_household(&household_repository, user.id(), HouseholdKind::Personal).await;
    let (shared_household, _) =
        insert_owned_household(&household_repository, user.id(), HouseholdKind::Shared).await;

    let result = household_repository
        .find_for_user(&user.id())
        .await
        .expect("Household repository lookup should succeed");

    assert!(result.contains(&personal_household));
    assert!(result.contains(&shared_household));
    assert_eq!(result.len(), 2);
}

#[sqlx::test]
async fn second_personal_household_for_same_user_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let user = insert_test_user(&user_repository).await;

    let (_, _) =
        insert_owned_household(&household_repository, user.id(), HouseholdKind::Personal).await;
    let (another_personal_household, another_personal_owner) =
        create_owned_household(user.id(), HouseholdKind::Personal);

    let result = household_repository
        .create_with_owner(&another_personal_household, &another_personal_owner)
        .await;

    assert_eq!(
        result,
        Err(HouseholdRepositoryError::PersonalHouseholdAlreadyExists)
    )
}

#[sqlx::test]
async fn duplicate_household_id_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let user = insert_test_user(&user_repository).await;

    let (household, owner) =
        insert_owned_household(&household_repository, user.id(), HouseholdKind::Shared).await;

    let result = household_repository
        .create_with_owner(&household, &owner)
        .await;

    assert_eq!(
        result,
        Err(HouseholdRepositoryError::HouseholdAlreadyExists)
    )
}

#[sqlx::test]
async fn transaction_rolls_back_if_owner_membership_insert_fails(pool: PgPool) {
    let household_repository = PostgresHouseholdRepository::new(pool);

    let non_existing_user = UserId::new();

    let (household, owner) = create_owned_household(non_existing_user, HouseholdKind::Shared);

    let result = household_repository
        .create_with_owner(&household, &owner)
        .await;

    assert!(result.is_err());

    let found = household_repository
        .find_by_id(&household.id())
        .await
        .expect("Household lookup should succeed");

    assert!(found.is_none())
}
