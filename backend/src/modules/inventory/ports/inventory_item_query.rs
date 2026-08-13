use async_trait::async_trait;

use crate::{
    modules::{households::domain::HouseholdId, inventory::read_models::InventoryItemListEntry},
    shared::db::PersistenceError,
};

#[async_trait]
pub trait InventoryItemQuery: Send + Sync {
    async fn find_active_for_household(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<InventoryItemListEntry>, InventoryItemQueryError>;
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum InventoryItemQueryError {
    #[error(transparent)]
    Persistence(#[from] PersistenceError),
    #[error("Invalid stored data")]
    InvalidStoredData,
}
