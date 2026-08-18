use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::{
    modules::{
        households::domain::HouseholdId,
        inventory::{
            domain::InventoryItemId,
            ports::{InventoryStockRepository, InventoryStockRepositoryError},
        },
    },
    shared::db::{PersistenceError, map_sqlx_error},
};

pub struct PostgresInventoryStockRepository {
    pool: PgPool,
}

struct StockStateRow {
    current_stock: i64,
    archived_at: Option<DateTime<Utc>>,
}

impl PostgresInventoryStockRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn find_stock_state(
        &self,
        household_id: &HouseholdId,
        item_id: &InventoryItemId,
    ) -> Result<Option<StockStateRow>, InventoryStockRepositoryError> {
        sqlx::query_as!(
            StockStateRow,
            r#"
            SELECT
                current_stock,
                archived_at
            FROM inventory_items
            WHERE id = $1 AND household_id = $2
            "#,
            item_id.as_uuid(),
            household_id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_stock_sqlx_error)
    }

    async fn map_failed_increase(
        &self,
        household_id: &HouseholdId,
        item_id: &InventoryItemId,
        amount: i64,
    ) -> Result<(), InventoryStockRepositoryError> {
        let state = self.find_stock_state(household_id, item_id).await?;

        match state {
            None => Err(InventoryStockRepositoryError::ItemNotFound),
            Some(state) if state.archived_at.is_some() => {
                Err(InventoryStockRepositoryError::ItemArchived)
            }
            Some(state) if state.current_stock > i64::from(u32::MAX) - amount => {
                Err(InventoryStockRepositoryError::StockOverflow)
            }
            Some(_) => {
                tracing::error!(
                    household_id = %household_id,
                    item_id = %item_id,
                    "Stock increase affected no rows unexpectedly",
                );
                Err(InventoryStockRepositoryError::Persistence(
                    PersistenceError::Failed,
                ))
            }
        }
    }

    async fn map_failed_decrease(
        &self,
        household_id: &HouseholdId,
        item_id: &InventoryItemId,
        amount: i64,
    ) -> Result<(), InventoryStockRepositoryError> {
        let state = self.find_stock_state(household_id, item_id).await?;

        match state {
            None => Err(InventoryStockRepositoryError::ItemNotFound),
            Some(state) if state.archived_at.is_some() => {
                Err(InventoryStockRepositoryError::ItemArchived)
            }
            Some(state) if state.current_stock < amount => {
                Err(InventoryStockRepositoryError::InsufficientStock)
            }
            Some(_) => {
                tracing::error!(
                    household_id = %household_id,
                    item_id = %item_id,
                    "Stock decrease affected no rows unexpectedly",
                );
                Err(InventoryStockRepositoryError::Persistence(
                    PersistenceError::Failed,
                ))
            }
        }
    }

    async fn map_missing_or_archived(
        &self,
        household_id: &HouseholdId,
        item_id: &InventoryItemId,
    ) -> Result<(), InventoryStockRepositoryError> {
        let state = self.find_stock_state(household_id, item_id).await?;

        match state {
            None => Err(InventoryStockRepositoryError::ItemNotFound),
            Some(state) if state.archived_at.is_some() => {
                Err(InventoryStockRepositoryError::ItemArchived)
            }
            Some(_) => {
                tracing::error!(
                    household_id = %household_id,
                    item_id = %item_id,
                    "Stock update affected no rows for an active inventory item"
                );
                Err(InventoryStockRepositoryError::Persistence(
                    PersistenceError::Failed,
                ))
            }
        }
    }
}

#[async_trait]
impl InventoryStockRepository for PostgresInventoryStockRepository {
    async fn increase(
        &self,
        household_id: &HouseholdId,
        item_id: &InventoryItemId,
        amount: u32,
        now: DateTime<Utc>,
    ) -> Result<(), InventoryStockRepositoryError> {
        let amount = i64::from(amount);

        let result = sqlx::query!(
            r#"
            UPDATE inventory_items
            SET
                current_stock = current_stock + $3,
                updated_at = $4
            WHERE id = $1
                AND household_id = $2
                AND archived_at IS NULL
                AND current_stock <= $5::BIGINT - $3::BIGINT
            "#,
            item_id.as_uuid(),
            household_id.as_uuid(),
            amount,
            now,
            i64::from(u32::MAX),
        )
        .execute(&self.pool)
        .await
        .map_err(map_stock_sqlx_error)?;

        if result.rows_affected() == 1 {
            return Ok(());
        }

        self.map_failed_increase(household_id, item_id, amount)
            .await
    }

    async fn decrease(
        &self,
        household_id: &HouseholdId,
        item_id: &InventoryItemId,
        amount: u32,
        now: DateTime<Utc>,
    ) -> Result<(), InventoryStockRepositoryError> {
        let amount = i64::from(amount);

        let result = sqlx::query!(
            r#"
            UPDATE inventory_items
            SET
                current_stock = current_stock - $3,
                updated_at = $4
            WHERE id = $1
                AND household_id = $2
                AND archived_at IS NULL
                AND current_stock >= $3
            "#,
            item_id.as_uuid(),
            household_id.as_uuid(),
            amount,
            now,
        )
        .execute(&self.pool)
        .await
        .map_err(map_stock_sqlx_error)?;

        if result.rows_affected() == 1 {
            return Ok(());
        }

        self.map_failed_decrease(household_id, item_id, amount)
            .await
    }

    async fn set(
        &self,
        household_id: &HouseholdId,
        item_id: &InventoryItemId,
        amount: u32,
        now: DateTime<Utc>,
    ) -> Result<(), InventoryStockRepositoryError> {
        let result = sqlx::query!(
            r#"
            UPDATE inventory_items
            SET
                current_stock = $3,
                updated_at = $4
            WHERE id = $1
                AND household_id = $2
                AND archived_at IS NULL
            "#,
            item_id.as_uuid(),
            household_id.as_uuid(),
            i64::from(amount),
            now,
        )
        .execute(&self.pool)
        .await
        .map_err(map_stock_sqlx_error)?;

        if result.rows_affected() == 1 {
            return Ok(());
        }

        self.map_missing_or_archived(household_id, item_id).await
    }
}

fn map_stock_sqlx_error(error: sqlx::Error) -> InventoryStockRepositoryError {
    InventoryStockRepositoryError::Persistence(map_sqlx_error(error))
}
