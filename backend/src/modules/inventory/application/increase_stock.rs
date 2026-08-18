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
            domain::InventoryItemId,
            ports::{InventoryStockRepository, InventoryStockRepositoryError},
        },
    },
    shared::application::InternalError,
};

pub struct IncreaseInventoryStockCommand {
    pub requester_id: UserId,
    pub household_id: HouseholdId,
    pub item_id: InventoryItemId,
    pub amount: u32,
}

pub struct IncreaseInventoryStockService {
    household_access_policy: Arc<dyn HouseholdAccessPolicy>,
    inventory_stock_repository: Arc<dyn InventoryStockRepository>,
}

impl IncreaseInventoryStockService {
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
        command: IncreaseInventoryStockCommand,
    ) -> Result<(), IncreaseInventoryStockError> {
        if command.amount == 0 {
            return Err(IncreaseInventoryStockError::InvalidAmount);
        }

        self.household_access_policy
            .require_member(&command.household_id, &command.requester_id)
            .await
            .map_err(map_household_access_error)?;

        self.inventory_stock_repository
            .increase(
                &command.household_id,
                &command.item_id,
                command.amount,
                Utc::now(),
            )
            .await
            .map_err(|error| match error {
                InventoryStockRepositoryError::ItemArchived => {
                    IncreaseInventoryStockError::ItemArchived
                }
                InventoryStockRepositoryError::ItemNotFound => {
                    IncreaseInventoryStockError::ItemNotFound
                }
                InventoryStockRepositoryError::StockOverflow => {
                    IncreaseInventoryStockError::StockOverflow
                }
                other => {
                    tracing::error!(
                        error = ?other,
                        household_id = %command.household_id,
                        item_id = %command.item_id,
                        amount = command.amount,
                        "Failed to increase inventory stock",
                    );
                    IncreaseInventoryStockError::Internal(InternalError::Failed)
                }
            })?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum IncreaseInventoryStockError {
    #[error("Inventory item was not found")]
    ItemNotFound,
    #[error("Inventory item is archived")]
    ItemArchived,
    #[error("Increase amount is invalid")]
    InvalidAmount,
    #[error("Stock can't be increased further")]
    StockOverflow,
    #[error("Household was not found")]
    HouseholdNotFound,
    #[error("You do not have permission")]
    Forbidden,
    #[error(transparent)]
    Internal(#[from] InternalError),
}

fn map_household_access_error(error: HouseholdAccessError) -> IncreaseInventoryStockError {
    match error {
        HouseholdAccessError::Forbidden => IncreaseInventoryStockError::Forbidden,
        HouseholdAccessError::HouseholdNotFound => IncreaseInventoryStockError::HouseholdNotFound,
        HouseholdAccessError::Internal(error) => IncreaseInventoryStockError::Internal(error),
    }
}

//TODO: Implement the in memory representation of the inventory_stock_repository and write the following tests
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn member_can_increase_inventory_stock() {}

    #[tokio::test]
    async fn zero_amount_is_rejected() {}

    #[tokio::test]
    async fn archived_item_cannot_be_increased() {}

    #[tokio::test]
    async fn increasing_above_u32_max_is_rejected() {}

    #[tokio::test]
    async fn unknown_item_returns_not_found() {}

    #[tokio::test]
    async fn non_member_is_forbidden() {}
}
*/
