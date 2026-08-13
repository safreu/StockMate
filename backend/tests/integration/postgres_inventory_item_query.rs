use backend::modules::{
    accounts::{adapters::PostgresUserRepository, ports::UserRepository},
    households::{
        adapters::PostgresHouseholdRepository,
        domain::{HouseholdId, HouseholdKind},
    },
    inventory::{
        adapters::{
            PostgresCategoryRepository, PostgresInventoryItemQuery, PostgresInventoryItemRepository,
        },
        domain::CategoryName,
        ports::{CategoryRepository, InventoryItemQuery, InventoryItemRepository},
        read_models::CategorySummary,
    },
};
use chrono::Utc;
use sqlx::PgPool;

use crate::integration::{
    builders::{CategoryTestBuilder, InventoryItemTestBuilder, UserTestBuilder},
    helpers::insert_owned_household,
};

#[sqlx::test]
async fn active_inventory_items_are_returned_with_category(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());

    let inventory_item_query = PostgresInventoryItemQuery::new(pool);

    let owner = UserTestBuilder::new().build();

    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let category = CategoryTestBuilder::new(household.id())
        .name("Food".to_owned())
        .build();

    category_repository
        .insert(&category)
        .await
        .expect("Category insertion should succeed");

    let item = InventoryItemTestBuilder::new(household.id())
        .name("Tofu".to_owned())
        .category(category.id())
        .build();

    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    let result = inventory_item_query
        .find_active_for_household(&household.id())
        .await
        .expect("Query should succeed");

    assert_eq!(
        result[0].category,
        Some(CategorySummary {
            name: CategoryName::parse("Food").expect("Category name should be valid"),
            id: category.id(),
        })
    );
}

#[sqlx::test]
async fn active_inventory_items_without_category_are_returned(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());

    let inventory_item_query = PostgresInventoryItemQuery::new(pool);

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
        .expect("Inventory item insertion should succeed");

    let result = inventory_item_query
        .find_active_for_household(&household.id())
        .await
        .expect("Query should succeed");

    assert!(result[0].category.is_none())
}

#[sqlx::test]
async fn archived_inventory_items_are_not_returned(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());

    let inventory_item_query = PostgresInventoryItemQuery::new(pool);

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
        .expect("Inventory item insertion should succeed");

    item.archive(Utc::now())
        .expect("Item archiving should succeed");

    inventory_item_repository
        .update(&item)
        .await
        .expect("Inventory item update should succeed");

    let result = inventory_item_query
        .find_active_for_household(&household.id())
        .await
        .expect("Query should succeed");

    assert!(result.is_empty())
}

#[sqlx::test]
async fn items_from_other_households_are_not_returned(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());

    let inventory_item_query = PostgresInventoryItemQuery::new(pool);

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
        .expect("Inventory item insertion should succeed");

    item.archive(Utc::now())
        .expect("Item archiving should succeed");

    inventory_item_repository
        .update(&item)
        .await
        .expect("Inventory item update should succeed");

    let result = inventory_item_query
        .find_active_for_household(&HouseholdId::new())
        .await
        .expect("Query should succeed");

    assert!(result.is_empty())
}
