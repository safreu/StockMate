use backend::modules::{
    accounts::{adapters::PostgresUserRepository, domain::UserId},
    households::{
        adapters::PostgresHouseholdRepository,
        domain::{HouseholdId, HouseholdKind, HouseholdMember, HouseholdName, HouseholdRole},
        ports::{HouseholdRepository, HouseholdRepositoryError},
    },
};
use chrono::{SubsecRound, Utc};
use sqlx::PgPool;

use crate::integration::helpers::{
    create_owned_household, insert_owned_household, insert_test_user, insert_test_user_with_email,
};

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

#[sqlx::test]
async fn existing_member_can_be_found(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let user = insert_test_user(&user_repository).await;

    let (household, owner) =
        insert_owned_household(&household_repository, user.id(), HouseholdKind::Shared).await;

    let result = household_repository
        .find_member(&household.id(), &user.id())
        .await
        .expect("Membership lookup should succeed");

    assert_eq!(result, Some(owner))
}

#[sqlx::test]
async fn unknown_member_returns_none(pool: PgPool) {
    let household_repository = PostgresHouseholdRepository::new(pool);

    let result = household_repository
        .find_member(&HouseholdId::new(), &UserId::new())
        .await
        .expect("Membership lookup should succeed");

    assert!(result.is_none())
}

#[sqlx::test]
async fn member_lookup_is_scoped_to_household(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let user = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, user.id(), HouseholdKind::Shared).await;

    let result = household_repository
        .find_member(&HouseholdId::new(), &user.id())
        .await
        .expect("Membership lookup should succeed");

    assert!(result.is_none());

    let original = household_repository
        .find_member(&household.id(), &user.id())
        .await
        .expect("Membership lookup should succeed");

    assert!(original.is_some())
}

#[sqlx::test]
async fn member_can_be_added_to_existing_household(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;
    let member = insert_test_user_with_email(&user_repository, "different.valid@email.com").await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let household_member = HouseholdMember::new(
        household.id(),
        member.id(),
        HouseholdRole::Member,
        Utc::now(),
    );

    let result = household_repository.add_member(&household_member).await;

    assert!(result.is_ok())
}

#[sqlx::test]
async fn added_member_can_be_found(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;
    let member = insert_test_user_with_email(&user_repository, "different.valid@email.com").await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let household_member = HouseholdMember::new(
        household.id(),
        member.id(),
        HouseholdRole::Member,
        Utc::now().trunc_subsecs(6),
    );

    household_repository
        .add_member(&household_member)
        .await
        .expect("Adding member should succeed");

    let stored = household_repository
        .find_member(&household.id(), &member.id())
        .await
        .expect("Membership lookup should succeed");

    assert_eq!(stored, Some(household_member))
}

#[sqlx::test]
async fn duplicate_member_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;
    let member = insert_test_user_with_email(&user_repository, "different.valid@email.com").await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let household_member = HouseholdMember::new(
        household.id(),
        member.id(),
        HouseholdRole::Member,
        Utc::now(),
    );

    household_repository
        .add_member(&household_member)
        .await
        .expect("Adding member should succeed");

    let result = household_repository.add_member(&household_member).await;

    assert_eq!(result, Err(HouseholdRepositoryError::MemberAlreadyExists))
}

#[sqlx::test]
async fn member_cannot_be_added_to_unknown_household(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;
    let member = insert_test_user_with_email(&user_repository, "different.valid@email.com").await;

    let (_, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let household_member = HouseholdMember::new(
        HouseholdId::new(),
        member.id(),
        HouseholdRole::Member,
        Utc::now(),
    );

    let result = household_repository.add_member(&household_member).await;

    assert_eq!(result, Err(HouseholdRepositoryError::HouseholdNotFound))
}

#[sqlx::test]
async fn existing_member_can_be_removed(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;
    let member = insert_test_user_with_email(&user_repository, "different.valid@email.com").await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let household_member = HouseholdMember::new(
        household.id(),
        member.id(),
        HouseholdRole::Member,
        Utc::now(),
    );

    household_repository
        .add_member(&household_member)
        .await
        .expect("Adding member should succeed");

    let result = household_repository
        .remove_member(&household.id(), &member.id())
        .await;

    assert!(result.is_ok())
}

#[sqlx::test]
async fn removed_member_can_no_longer_be_found(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;
    let member = insert_test_user_with_email(&user_repository, "different.valid@email.com").await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let household_member = HouseholdMember::new(
        household.id(),
        member.id(),
        HouseholdRole::Member,
        Utc::now(),
    );

    household_repository
        .add_member(&household_member)
        .await
        .expect("Adding member should succeed");

    household_repository
        .remove_member(&household.id(), &member.id())
        .await
        .expect("Removing member should succeed");

    let result = household_repository
        .find_member(&household.id(), &member.id())
        .await
        .expect("Membership lookup should succeed");

    assert!(result.is_none())
}

#[sqlx::test]
async fn removing_unknown_member_returns_not_found(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;
    let member = insert_test_user_with_email(&user_repository, "different.valid@email.com").await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let result = household_repository
        .remove_member(&household.id(), &member.id())
        .await;

    assert_eq!(result, Err(HouseholdRepositoryError::MemberNotFound))
}

#[sqlx::test]
async fn updated_household_can_be_loaded(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (mut household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    household.rename(
        HouseholdName::parse("New name").expect("New name should be valid"),
        Utc::now().trunc_subsecs(6),
    );

    household_repository
        .update(&household)
        .await
        .expect("Renaming of household should succeed");

    let result = household_repository
        .find_by_id(&household.id())
        .await
        .expect("Household lookup should succeed");

    assert_eq!(result, Some(household))
}

#[sqlx::test]
async fn updating_unknown_household_returns_household_not_found(pool: PgPool) {
    let household_repository = PostgresHouseholdRepository::new(pool);

    let (mut household, _) = create_owned_household(UserId::new(), HouseholdKind::Shared);

    household.rename(
        HouseholdName::parse("New name").expect("New name should be valid"),
        Utc::now().trunc_subsecs(6),
    );

    let result = household_repository.update(&household).await;

    assert_eq!(result, Err(HouseholdRepositoryError::HouseholdNotFound))
}
