use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    modules::{
        households::adapters::{DefaultHouseholdAccessPolicy, PostgresHouseholdRepository},
        inventory::{
            adapters::{
                PostgresCategoryRepository, PostgresInventoryItemQuery,
                PostgresInventoryItemRepository,
            },
            application::{
                CreateCategoryService, CreateInventoryItemService, DeleteCategoryService,
                ListCategoriesService, ListInventoryItemsService,
            },
        },
    },
    shared::api::InventoryItemState,
};

pub(super) fn build_inventory_item_state(pool: &PgPool) -> InventoryItemState {
    let household_repository = Arc::new(PostgresHouseholdRepository::new(pool.clone()));
    let household_access_policy = Arc::new(DefaultHouseholdAccessPolicy::new(household_repository));
    let category_repository = Arc::new(PostgresCategoryRepository::new(pool.clone()));
    let inventory_item_repository = Arc::new(PostgresInventoryItemRepository::new(pool.clone()));
    let inventory_item_query = Arc::new(PostgresInventoryItemQuery::new(pool.clone()));

    let create_inventory_item_service = Arc::new(CreateInventoryItemService::new(
        household_access_policy.clone(),
        category_repository.clone(),
        inventory_item_repository,
    ));

    let create_category_service = Arc::new(CreateCategoryService::new(
        household_access_policy.clone(),
        category_repository.clone(),
    ));

    let list_categories_service = Arc::new(ListCategoriesService::new(
        household_access_policy.clone(),
        category_repository.clone(),
    ));

    let delete_category_service = Arc::new(DeleteCategoryService::new(
        household_access_policy.clone(),
        category_repository,
    ));

    let list_inventory_items_service = Arc::new(ListInventoryItemsService::new(
        household_access_policy,
        inventory_item_query,
    ));

    InventoryItemState {
        create_inventory_item: create_inventory_item_service,
        create_category: create_category_service,
        list_categories: list_categories_service,
        delete_category: delete_category_service,
        list_inventory_items: list_inventory_items_service,
    }
}
