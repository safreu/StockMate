use backend::modules::{accounts::adapters::PostgresUserRepository, households::adapters::PostgresHouseholdRepository, inventory::adapters::PostgresCategoryRepository};
use sqlx::PgPool;


#[sqlx::test]
async fn inventory_item_can_be_inserted_and_loaded(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let household_repository = PostgresHouseholdRepository::new(pool.clone());
    let category_repository = PostgresCategoryRepository::new(pool);
    
}

#[sqlx::test]
async fn duplicate_inventory_item_id_is_rejected(pool: PgPool) {}

#[sqlx::test]
async fn duplicate_active_normalized_name_in_same_household_is_rejected(pool: PgPool) {}

#[sqlx::test]
async fn same_normalized_name_in_different_households_is_allowed(pool: PgPool) {}

#[sqlx::test]
async fn archived_item_does_not_block_reusing_its_name(pool: PgPool) {}

#[sqlx::test]
async fn active_item_can_be_found_by_normalized_name(pool: PgPool) {}

#[sqlx::test]
async fn archived_item_is_not_found_by_active_name_lookup(pool: PgPool) {}

#[sqlx::test]
async fn find_active_for_household_returns_only_active_items_of_that_household(pool: PgPool) {}

#[sqlx::test]
async fn find_archived_for_household_returns_only_archived_items_of_that_household(pool: PgPool) {}

#[sqlx::test]
async fn existing_inventory_item_can_be_updated(pool: PgPool) {}

#[sqlx::test]
async fn updating_inventory_item_to_duplicate_active_name_is_rejected(pool: PgPool) {}

#[sqlx::test]
async fn restoring_item_with_duplicate_active_name_is_rejected(pool: PgPool) {}

#[sqlx::test]
async fn updating_unknown_inventory_item_returns_not_found(pool: PgPool) {}