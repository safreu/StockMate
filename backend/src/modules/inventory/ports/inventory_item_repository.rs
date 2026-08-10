use async_trait::async_trait;

use crate::{
    modules::{
        households::domain::HouseholdId,
        inventory::domain::{InventoryItem, InventoryItemId, InventoryItemName},
    },
    shared::db::PersistenceError,
};

#[async_trait]
pub trait InventoryItemRepository: Send + Sync {
    async fn insert(&self, item: &InventoryItem) -> Result<(), InventoryItemRepositoryError>;

    async fn find_by_id(
        &self,
        id: &InventoryItemId,
    ) -> Result<Option<InventoryItem>, InventoryItemRepositoryError>;

    async fn find_active_by_name(
        &self,
        household_id: &HouseholdId,
        name: &InventoryItemName,
    ) -> Result<Option<InventoryItem>, InventoryItemRepositoryError>;

    async fn find_for_household(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<InventoryItem>, InventoryItemRepositoryError>;

    async fn update(&self, item: &InventoryItem) -> Result<(), InventoryItemRepositoryError>;
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum InventoryItemRepositoryError {
    #[error("Inventory item already exists")]
    ItemAlreadyExists,
    #[error("Inventory item not found")]
    ItemNotFound,
    #[error("Category does not exist")]
    CategoryNotFound,
    #[error("Invalid stored inventory item data")]
    InvalidStoredData,
    #[error(transparent)]
    Persistence(#[from] PersistenceError),
}
