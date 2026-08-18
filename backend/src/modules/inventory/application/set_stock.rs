use std::sync::Arc;

use chrono::Utc;

use crate::{
    modules::{
        accounts::domain::UserId,
        households::{
            domain::HouseholdId,
            ports::{HouseholdAccessError, HouseholdAccessPolicy},
        },
        inventory::{
            domain::{InventoryItemId, InventoryStockEventSource},
            ports::{
                InventoryStockRepository, InventoryStockRepositoryError, StockMutationContext,
            },
        },
    },
    shared::application::InternalError,
};

pub struct SetInventoryStockCommand {
    pub requester_id: UserId,
    pub household_id: HouseholdId,
    pub item_id: InventoryItemId,
    pub stock: u32,
}

pub struct SetInventoryStockService {
    household_access_policy: Arc<dyn HouseholdAccessPolicy>,
    inventory_stock_repository: Arc<dyn InventoryStockRepository>,
}

impl SetInventoryStockService {
    pub fn new(
        household_access_policy: Arc<dyn HouseholdAccessPolicy>,
        inventory_stock_repository: Arc<dyn InventoryStockRepository>,
    ) -> Self {
        Self {
            household_access_policy,
            inventory_stock_repository,
        }
    }

    pub async fn execute(
        &self,
        command: SetInventoryStockCommand,
    ) -> Result<(), SetInventoryStockError> {
        self.household_access_policy
            .require_member(&command.household_id, &command.requester_id)
            .await
            .map_err(map_household_access_error)?;

        //TODO: When implemented replace this with actual source and device_id
        let context = StockMutationContext {
            actor_user_id: Some(command.requester_id),
            actor_device_id: None,
            source: InventoryStockEventSource::Manual,
        };

        self.inventory_stock_repository
            .set(
                &command.household_id,
                &command.item_id,
                command.stock,
                &context,
                Utc::now(),
            )
            .await
            .map_err(|error| match error {
                InventoryStockRepositoryError::ItemArchived => SetInventoryStockError::ItemArchived,
                InventoryStockRepositoryError::ItemNotFound => SetInventoryStockError::ItemNotFound,
                other => {
                    tracing::error!(
                        error = ?other,
                        household_id = %command.household_id,
                        item_id = %command.item_id,
                        amount = command.stock,
                        "Failed to set inventory stock",
                    );
                    SetInventoryStockError::Internal(InternalError::Failed)
                }
            })?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum SetInventoryStockError {
    #[error("Inventory item was not found")]
    ItemNotFound,
    #[error("Inventory item is archived")]
    ItemArchived,
    #[error("Household was not found")]
    HouseholdNotFound,
    #[error("You do not have permission")]
    Forbidden,
    #[error(transparent)]
    Internal(#[from] InternalError),
}

fn map_household_access_error(error: HouseholdAccessError) -> SetInventoryStockError {
    match error {
        HouseholdAccessError::Forbidden => SetInventoryStockError::Forbidden,
        HouseholdAccessError::HouseholdNotFound => SetInventoryStockError::HouseholdNotFound,
        HouseholdAccessError::Internal(error) => SetInventoryStockError::Internal(error),
    }
}

//TODO: Implement the in memory representation of the inventory_stock_repository and write the following tests
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn member_can_set_inventory_stock() {}

    #[tokio::test]
    async fn stock_can_be_set_to_zero() {}

    #[tokio::test]
    async fn archived_item_stock_cannot_be_set() {}

    #[tokio::test]
    async fn unknown_item_returns_not_found() {}

    #[tokio::test]
    async fn non_member_is_forbidden() {}
}
*/
