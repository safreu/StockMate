use backend::modules::{
    accounts::adapters::PostgresUserRepository,
    households::{adapters::PostgresHouseholdRepository, domain::HouseholdKind},
    inventory::{
        adapters::PostgresCategoryRepository,
        domain::{Category, CategoryId, CategoryName},
        ports::{CategoryRepository, CategoryRepositoryError},
    },
};
use chrono::{SubsecRound, Utc};
use sqlx::PgPool;

use crate::integration::helpers::{insert_owned_household, insert_test_user, test_category};

#[sqlx::test]
async fn category_can_be_inserted_and_loaded(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let category = test_category(household.id(), "Fruit");

    let result = category_repository.insert(&category).await;

    assert!(result.is_ok());

    let stored = category_repository
        .find_by_id(&category.id(), &household.id())
        .await
        .expect("Category lookup should succeed");

    assert_eq!(stored, Some(category))
}

#[sqlx::test]
async fn duplicate_category_id_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let category = test_category(household.id(), "Fruit");

    let another_category = Category::new(
        category.id(),
        household.id(),
        CategoryName::parse("vegetables").expect("Category name should be valid"),
        Utc::now(),
        Utc::now(),
    );

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");

    let result = category_repository.insert(&another_category).await;

    assert_eq!(result, Err(CategoryRepositoryError::CategoryAlreadyExists))
}

#[sqlx::test]
async fn duplicate_normalized_name_in_same_household_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let category = test_category(household.id(), "Fruit");

    let another_category = test_category(household.id(), "fruit");

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");

    let result = category_repository.insert(&another_category).await;

    assert_eq!(result, Err(CategoryRepositoryError::CategoryAlreadyExists))
}

#[sqlx::test]
async fn same_normalized_name_in_different_households_is_allowed(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;
    let (another_household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let category = test_category(household.id(), "Fruit");

    let another_category = test_category(another_household.id(), "fruit");

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");

    let result = category_repository.insert(&another_category).await;

    assert!(result.is_ok())
}

#[sqlx::test]
async fn category_can_be_found_by_normalized_name(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let category = test_category(household.id(), "Fruit");

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");

    let stored = category_repository
        .find_by_name(
            &household.id(),
            &CategoryName::parse("fruit").expect("Category name should be valid"),
        )
        .await
        .expect("Category lookup should succeed");

    assert_eq!(stored, Some(category))
}

#[sqlx::test]
async fn find_for_household_returns_only_categories_for_that_household(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;
    let (another_household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let category = test_category(household.id(), "Fruit");
    let another_category = test_category(another_household.id(), "Fruit");

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");
    category_repository
        .insert(&another_category)
        .await
        .expect("Category insertion should succeed");

    let stored = category_repository
        .find_for_household(&household.id())
        .await
        .expect("Category lookup should succeed");

    assert_eq!(stored.len(), 1);
    assert!(stored.contains(&category));
    assert!(!stored.contains(&another_category));
}

#[sqlx::test]
async fn existing_category_can_be_updated(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut category = test_category(household.id(), "Fruit");

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");

    let now = Utc::now().trunc_subsecs(6);

    category.rename(
        CategoryName::parse("Vegetables").expect("Name should be valid"),
        now,
    );

    category_repository
        .update(&category)
        .await
        .expect("Category update should succeed");

    let stored = category_repository
        .find_by_id(&category.id(), &household.id())
        .await
        .expect("Category lookup should succeed")
        .expect("Category should exist");

    assert_eq!(stored.name().normalized(), "vegetables");
    assert_eq!(stored.updated_at(), now);
}

#[sqlx::test]
async fn updating_category_to_duplicate_name_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut category = test_category(household.id(), "Fruit");
    let another_category = test_category(household.id(), "Vegetables");

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");
    category_repository
        .insert(&another_category)
        .await
        .expect("Category insertion should succeed");

    let now = Utc::now();

    category.rename(
        CategoryName::parse("vegetables").expect("Name should be valid"),
        now,
    );

    let result = category_repository.update(&category).await;

    assert_eq!(result, Err(CategoryRepositoryError::CategoryAlreadyExists))
}

#[sqlx::test]
async fn updating_unknown_category_returns_not_found(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut category = test_category(household.id(), "Fruit");

    let now = Utc::now();

    category.rename(
        CategoryName::parse("Vegetables").expect("Name should be valid"),
        now,
    );

    let result = category_repository.update(&category).await;

    assert_eq!(result, Err(CategoryRepositoryError::CategoryNotFound))
}

#[sqlx::test]
async fn existing_category_can_be_deleted(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let category = test_category(household.id(), "Fruit");

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");

    category_repository
        .delete(&category.id(), &household.id())
        .await
        .expect("Category deletion should succeed");

    let result = category_repository
        .find_by_id(&category.id(), &household.id())
        .await
        .expect("Category lookup should succeed");

    assert!(result.is_none())
}

#[sqlx::test]
async fn deleting_category_from_different_household_returns_not_found(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;
    let (another_household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let category = test_category(household.id(), "Fruit");
    let another_category = test_category(another_household.id(), "vegetables");

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");
    category_repository
        .insert(&another_category)
        .await
        .expect("Category insertion should succeed");

    let result = category_repository
        .delete(&another_category.id(), &household.id())
        .await;

    assert_eq!(result, Err(CategoryRepositoryError::CategoryNotFound))
}

#[sqlx::test]
async fn deleting_unknown_category_returns_not_found(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);

    let owner = insert_test_user(&user_repository).await;

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let category = test_category(household.id(), "Fruit");

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");

    let result = category_repository
        .delete(&CategoryId::new(), &household.id())
        .await;

    assert_eq!(result, Err(CategoryRepositoryError::CategoryNotFound))
}
