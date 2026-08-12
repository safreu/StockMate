use crate::integration::builders::{InventoryItemTestBuilder, UserTestBuilder};
use backend::modules::{
    accounts::{adapters::PostgresUserRepository, ports::UserRepository},
    households::{adapters::PostgresHouseholdRepository, domain::HouseholdKind},
    inventory::{
        adapters::PostgresInventoryItemRepository,
        domain::InventoryItemName,
        ports::{InventoryItemRepository, InventoryItemRepositoryError},
    },
};
use chrono::{SubsecRound, Utc};
use sqlx::PgPool;

use crate::integration::helpers::insert_owned_household;

#[sqlx::test]
async fn inventory_item_can_be_inserted_and_loaded(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id()).build();

    let result = inventory_item_repository.insert(&item).await;

    assert!(result.is_ok());

    let stored = inventory_item_repository
        .find_by_id(&item.id(), &household.id())
        .await
        .expect("Inventory item lookup should succeed");

    assert_eq!(stored, Some(item))
}

#[sqlx::test]
async fn duplicate_inventory_item_id_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id()).build();
    let another_item = InventoryItemTestBuilder::new(household.id())
        .id(item.id())
        .name("Tofu".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");
    let result = inventory_item_repository.insert(&another_item).await;

    assert_eq!(result, Err(InventoryItemRepositoryError::ItemAlreadyExists))
}

#[sqlx::test]
async fn duplicate_active_normalized_name_in_same_household_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();
    let another_item = InventoryItemTestBuilder::new(household.id())
        .name("tofu".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");
    let result = inventory_item_repository.insert(&another_item).await;

    assert_eq!(result, Err(InventoryItemRepositoryError::ItemAlreadyExists))
}

#[sqlx::test]
async fn same_normalized_name_in_different_households_is_allowed(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;
    let (another_household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();
    let another_item = InventoryItemTestBuilder::new(another_household.id())
        .name("tofu".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");
    let result = inventory_item_repository.insert(&another_item).await;

    assert!(result.is_ok())
}

#[sqlx::test]
async fn archived_item_does_not_block_reusing_its_name(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();
    let another_item = InventoryItemTestBuilder::new(household.id())
        .name("tofu".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    item.archive(Utc::now().trunc_subsecs(6))
        .expect("Inventory item archiving should succeed");
    inventory_item_repository
        .update(&item)
        .await
        .expect("Inventory item updating should succeed");

    let result = inventory_item_repository.insert(&another_item).await;

    assert!(result.is_ok())
}

#[sqlx::test]
async fn active_item_can_be_found_by_normalized_name(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Item insertion should succeed");

    let stored = inventory_item_repository
        .find_active_by_name(
            &household.id(),
            &InventoryItemName::parse("tofu").expect("Inventory item name should be valid"),
        )
        .await
        .expect("Inventory item lookup should succeed");

    assert_eq!(stored, Some(item))
}

#[sqlx::test]
async fn archived_item_is_not_found_by_active_name_lookup(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Item insertion should succeed");

    item.archive(Utc::now().trunc_subsecs(6))
        .expect("Inventory item archiving should succeed");
    inventory_item_repository
        .update(&item)
        .await
        .expect("Inventory item updating should succeed");

    let stored = inventory_item_repository
        .find_active_by_name(
            &household.id(),
            &InventoryItemName::parse("tofu").expect("Inventory item name should be valid"),
        )
        .await
        .expect("Inventory item lookup should succeed");

    assert!(stored.is_none())
}

#[sqlx::test]
async fn find_active_for_household_returns_only_active_items_of_that_household(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Item insertion should succeed");

    item.archive(Utc::now().trunc_subsecs(6))
        .expect("Inventory item archiving should succeed");
    inventory_item_repository
        .update(&item)
        .await
        .expect("Inventory item updating should succeed");

    let another_item = InventoryItemTestBuilder::new(household.id())
        .name("Apple".to_owned())
        .build();
    inventory_item_repository
        .insert(&another_item)
        .await
        .expect("Item insertion should succeed");

    let stored = inventory_item_repository
        .find_active_for_household(&household.id())
        .await
        .expect("Inventory item lookup should succeed");

    assert_eq!(stored.len(), 1);
    assert!(stored.contains(&another_item));
    assert!(!stored.contains(&item));
}

#[sqlx::test]
async fn find_archived_for_household_returns_only_archived_items_of_that_household(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Item insertion should succeed");

    item.archive(Utc::now().trunc_subsecs(6))
        .expect("Inventory item archiving should succeed");

    inventory_item_repository
        .update(&item)
        .await
        .expect("Inventory item updating should succeed");

    let another_item = InventoryItemTestBuilder::new(household.id())
        .name("Apple".to_owned())
        .build();

    inventory_item_repository
        .insert(&another_item)
        .await
        .expect("Item insertion should succeed");

    let stored = inventory_item_repository
        .find_archived_for_household(&household.id())
        .await
        .expect("Inventory item lookup should succeed");

    assert_eq!(stored.len(), 1);
    assert!(!stored.contains(&another_item));
    assert!(stored.contains(&item));
}

#[sqlx::test]
async fn existing_inventory_item_can_be_updated(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Item insertion should succeed");

    let now = Utc::now().trunc_subsecs(6).trunc_subsecs(6);

    item.rename(
        InventoryItemName::parse("Apple").expect("Inventory item name should be valid"),
        now,
    )
    .expect("Inventory item renaming should succeed");
    let update = inventory_item_repository.update(&item).await;

    assert!(update.is_ok());

    let stored = inventory_item_repository
        .find_by_id(&item.id(), &household.id())
        .await
        .expect("Inventory item lookup should succeed")
        .expect("Inventory item should exist");

    assert_eq!(stored.updated_at(), now);
    assert_eq!(stored.name().as_str(), "Apple")
}

#[sqlx::test]
async fn updating_inventory_item_to_duplicate_active_name_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();
    let another_item = InventoryItemTestBuilder::new(household.id())
        .name("apple".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Item insertion should succeed");
    inventory_item_repository
        .insert(&another_item)
        .await
        .expect("Item insertion should succeed");

    let now = Utc::now().trunc_subsecs(6);

    item.rename(
        InventoryItemName::parse("Apple").expect("Inventory item name should be valid"),
        now,
    )
    .expect("Inventory item renaming should succeed");
    let update = inventory_item_repository.update(&item).await;

    assert_eq!(update, Err(InventoryItemRepositoryError::ItemAlreadyExists));
}

#[sqlx::test]
async fn restoring_item_with_duplicate_active_name_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();
    let another_item = InventoryItemTestBuilder::new(household.id())
        .name("tofu".to_owned())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    item.archive(Utc::now().trunc_subsecs(6))
        .expect("Inventory item archiving should succeed");
    inventory_item_repository
        .update(&item)
        .await
        .expect("Inventory item updating should succeed");

    inventory_item_repository
        .insert(&another_item)
        .await
        .expect("Inventory item insertion should succeed");

    item.restore(Utc::now().trunc_subsecs(6))
        .expect("Inventory item restoration should succeed");
    let result = inventory_item_repository.update(&item).await;

    assert_eq!(result, Err(InventoryItemRepositoryError::ItemAlreadyExists))
}

#[sqlx::test]
async fn updating_unknown_inventory_item_returns_not_found(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .build();

    item.rename(
        InventoryItemName::parse("Apple").expect("Inventory item name should be valid"),
        Utc::now().trunc_subsecs(6),
    )
    .expect("Inventory item renaming should succeed");
    let update = inventory_item_repository.update(&item).await;

    assert_eq!(update, Err(InventoryItemRepositoryError::ItemNotFound));
}
