use std::sync::Arc;

use backend::modules::{
    accounts::{adapters::PostgresUserRepository, ports::UserRepository},
    households::{adapters::PostgresHouseholdRepository, domain::HouseholdKind},
    inventory::{
        self,
        adapters::{PostgresInventoryItemRepository, PostgresInventoryStockRepository},
        domain::InventoryItemId,
        ports::{InventoryItemRepository, InventoryStockRepository},
    },
};
use chrono::Utc;
use sqlx::PgPool;

use crate::integration::{
    builders::{InventoryItemTestBuilder, UserTestBuilder},
    helpers::insert_owned_household,
};

#[sqlx::test]
async fn stock_can_be_increased(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());
    let stock_repository = Arc::new(PostgresInventoryStockRepository::new(pool.clone()));

    let owner = UserTestBuilder::new().build();
    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id())
        .current_stock(0)
        .name("Tofu".to_owned())
        .build();
    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    stock_repository
        .increase(&household.id(), &item.id(), 1, Utc::now())
        .await
        .expect("Inventory item stock increase should succeed");

    let stored = inventory_item_repository
        .find_by_id(&item.id(), &household.id())
        .await
        .expect("Inventory item lookup should succeed")
        .expect("Inventory item should exists");

    assert_eq!(stored.current_stock(), 1)
}

#[sqlx::test]
async fn stock_can_be_decreased(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());
    let stock_repository = Arc::new(PostgresInventoryStockRepository::new(pool.clone()));

    let owner = UserTestBuilder::new().build();
    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id())
        .current_stock(1)
        .name("Tofu".to_owned())
        .build();
    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    stock_repository
        .decrease(&household.id(), &item.id(), 1, Utc::now())
        .await
        .expect("Inventory item stock decrease should succeed");

    let stored = inventory_item_repository
        .find_by_id(&item.id(), &household.id())
        .await
        .expect("Inventory item lookup should succeed")
        .expect("Inventory item should exists");

    assert_eq!(stored.current_stock(), 0)
}

#[sqlx::test]
async fn stock_can_be_set(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());
    let stock_repository = Arc::new(PostgresInventoryStockRepository::new(pool.clone()));

    let owner = UserTestBuilder::new().build();
    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id())
        .current_stock(1)
        .name("Tofu".to_owned())
        .build();
    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    stock_repository
        .set(&household.id(), &item.id(), 10, Utc::now())
        .await
        .expect("Inventory item stock decrease should succeed");

    let stored = inventory_item_repository
        .find_by_id(&item.id(), &household.id())
        .await
        .expect("Inventory item lookup should succeed")
        .expect("Inventory item should exists");

    assert_eq!(stored.current_stock(), 10)
}

#[sqlx::test]
async fn decreasing_below_zero_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());
    let stock_repository = Arc::new(PostgresInventoryStockRepository::new(pool.clone()));

    let owner = UserTestBuilder::new().build();
    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id())
        .current_stock(0)
        .name("Tofu".to_owned())
        .build();
    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    let result = stock_repository
        .decrease(&household.id(), &item.id(), 1, Utc::now())
        .await;

    assert_eq!(
        result,
        Err(inventory::ports::InventoryStockRepositoryError::InsufficientStock)
    )
}

#[sqlx::test]
async fn increasing_above_u32_max_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());
    let stock_repository = Arc::new(PostgresInventoryStockRepository::new(pool.clone()));

    let owner = UserTestBuilder::new().build();
    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id())
        .current_stock(u32::MAX)
        .name("Tofu".to_owned())
        .build();
    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    let result = stock_repository
        .increase(&household.id(), &item.id(), 1, Utc::now())
        .await;

    assert_eq!(
        result,
        Err(inventory::ports::InventoryStockRepositoryError::StockOverflow)
    )
}

#[sqlx::test]
async fn archived_item_cannot_be_modified(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());
    let stock_repository = Arc::new(PostgresInventoryStockRepository::new(pool.clone()));

    let owner = UserTestBuilder::new().build();
    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let mut item = InventoryItemTestBuilder::new(household.id())
        .current_stock(0)
        .name("Tofu".to_owned())
        .build();
    item.archive(Utc::now())
        .expect("Item archiving should succeed");
    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    let result = stock_repository
        .increase(&household.id(), &item.id(), 1, Utc::now())
        .await;

    assert_eq!(
        result,
        Err(inventory::ports::InventoryStockRepositoryError::ItemArchived)
    )
}

#[sqlx::test]
async fn unknown_item_returns_not_found(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());
    let stock_repository = Arc::new(PostgresInventoryStockRepository::new(pool.clone()));

    let owner = UserTestBuilder::new().build();
    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id())
        .current_stock(1)
        .name("Tofu".to_owned())
        .build();
    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    let result = stock_repository
        .decrease(&household.id(), &InventoryItemId::new(), 1, Utc::now())
        .await;

    assert_eq!(
        result,
        Err(inventory::ports::InventoryStockRepositoryError::ItemNotFound)
    )
}

#[sqlx::test]
async fn item_from_different_household_returns_not_found(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());
    let stock_repository = Arc::new(PostgresInventoryStockRepository::new(pool.clone()));

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
        .current_stock(1)
        .name("Tofu".to_owned())
        .build();
    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    let result = stock_repository
        .decrease(&another_household.id(), &item.id(), 1, Utc::now())
        .await;

    assert_eq!(
        result,
        Err(inventory::ports::InventoryStockRepositoryError::ItemNotFound)
    )
}

#[sqlx::test]
async fn concurrent_stock_increases_are_atomic(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let inventory_item_repository = PostgresInventoryItemRepository::new(pool.clone());
    let stock_repository = Arc::new(PostgresInventoryStockRepository::new(pool.clone()));

    let owner = UserTestBuilder::new().build();
    user_repository
        .insert(&owner)
        .await
        .expect("User insertion should succeed");

    let (household, _) =
        insert_owned_household(&household_repository, owner.id(), HouseholdKind::Shared).await;

    let item = InventoryItemTestBuilder::new(household.id())
        .current_stock(0)
        .name("Tofu".to_owned())
        .build();
    inventory_item_repository
        .insert(&item)
        .await
        .expect("Inventory item insertion should succeed");

    let mut tasks = Vec::new();

    for _ in 0..100 {
        let repository = stock_repository.clone();
        let household_id = household.id();
        let item_id = item.id();

        tasks.push(tokio::spawn(async move {
            repository
                .increase(&household_id, &item_id, 1, Utc::now())
                .await
        }));
    }

    for task in tasks {
        task.await
            .expect("Stock increase task should not panic")
            .expect("Stock increase should succeed");
    }

    let stored = inventory_item_repository
        .find_by_id(&item.id(), &household.id())
        .await
        .expect("Inventory item lookup should succeed")
        .expect("Inventory item should exists");

    assert_eq!(stored.current_stock(), 100)
}
