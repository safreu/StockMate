use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    modules::{
        households::adapters::PostgresHouseholdRepository,
        inventory::{
            adapters::{PostgresCategoryRepository, PostgresInventoryItemRepository},
            application::CreateInventoryItemService,
        },
    },
    shared::api::InventoryItemState,
};

pub(super) fn build_inventory_item_state(pool: &PgPool) -> InventoryItemState {
    let household_repository = Arc::new(PostgresHouseholdRepository::new(pool.clone()));
    let category_repository = Arc::new(PostgresCategoryRepository::new(pool.clone()));
    let inventory_item_repository = Arc::new(PostgresInventoryItemRepository::new(pool.clone()));

    let create_inventory_item_service = Arc::new(CreateInventoryItemService::new(
        household_repository,
        category_repository,
        inventory_item_repository,
    ));

    InventoryItemState {
        crate_inventory_item: create_inventory_item_service,
    }
}
