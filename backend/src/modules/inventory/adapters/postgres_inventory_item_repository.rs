use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    modules::{
        households::domain::HouseholdId,
        inventory::{
            domain::{
                CategoryId, InventoryItem, InventoryItemId, InventoryItemName, InventoryPriority,
            },
            ports::{InventoryItemRepository, InventoryItemRepositoryError},
        },
    },
    shared::db::map_sqlx_error,
};

pub struct PostgresInventoryItemRepository {
    pool: PgPool,
}

impl PostgresInventoryItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InventoryItemRepository for PostgresInventoryItemRepository {
    async fn insert(&self, item: &InventoryItem) -> Result<(), InventoryItemRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO inventory_items (
                id,
                household_id,
                category_id,
                name,
                normalized_name,
                current_stock,
                reorder_threshold,
                priority,
                archived_at,
                created_at,
                updated_at
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            item.id().into_uuid(),
            item.household_id().into_uuid(),
            item.category_id().map(CategoryId::into_uuid),
            item.name().as_str(),
            item.name().normalized(),
            i64::from(item.current_stock()),
            i64::from(item.reorder_threshold()),
            item.priority().as_str(),
            item.archived_at(),
            item.created_at(),
            item.updated_at(),
        )
        .execute(&self.pool)
        .await
        .map_err(map_write_inventory_item_error)?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &InventoryItemId,
        household_id: &HouseholdId,
    ) -> Result<Option<InventoryItem>, InventoryItemRepositoryError> {
        let row = sqlx::query_as!(
            InventoryItemRow,
            r#"
            SELECT
                id,
                household_id,
                category_id,
                name,
                current_stock,
                reorder_threshold,
                priority,
                archived_at,
                created_at,
                updated_at
            FROM inventory_items
            WHERE id = $1 AND household_id = $2
            "#,
            id.into_uuid(),
            household_id.into_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        row.map(InventoryItem::try_from).transpose()
    }

    async fn find_active_by_name(
        &self,
        household_id: &HouseholdId,
        name: &InventoryItemName,
    ) -> Result<Option<InventoryItem>, InventoryItemRepositoryError> {
        let row = sqlx::query_as!(
            InventoryItemRow,
            r#"
            SELECT
                id,
                household_id,
                category_id,
                name,
                current_stock,
                reorder_threshold,
                priority,
                archived_at,
                created_at,
                updated_at
            FROM inventory_items
            WHERE household_id = $1 AND normalized_name = $2 AND archived_at IS NULL
            "#,
            household_id.into_uuid(),
            name.normalized(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        row.map(InventoryItem::try_from).transpose()
    }

    async fn find_active_for_household(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<InventoryItem>, InventoryItemRepositoryError> {
        let rows = sqlx::query_as!(
            InventoryItemRow,
            r#"
            SELECT
                id,
                household_id,
                category_id,
                name,
                current_stock,
                reorder_threshold,
                priority,
                archived_at,
                created_at,
                updated_at
            FROM inventory_items
            WHERE household_id = $1 AND archived_at IS NULL
            "#,
            household_id.into_uuid(),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        rows.into_iter().map(InventoryItem::try_from).collect()
    }

    async fn find_archived_for_household(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<InventoryItem>, InventoryItemRepositoryError> {
        let rows = sqlx::query_as!(
            InventoryItemRow,
            r#"
            SELECT
                id,
                household_id,
                category_id,
                name,
                current_stock,
                reorder_threshold,
                priority,
                archived_at,
                created_at,
                updated_at
            FROM inventory_items
            WHERE household_id = $1 AND archived_at IS NOT NULL
            "#,
            household_id.into_uuid(),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        rows.into_iter().map(InventoryItem::try_from).collect()
    }

    async fn update(&self, item: &InventoryItem) -> Result<(), InventoryItemRepositoryError> {
        let result = sqlx::query!(
            r#"
            UPDATE inventory_items
            SET
                category_id = $3,
                name = $4,
                normalized_name = $5,
                current_stock = $6,
                reorder_threshold = $7,
                priority = $8,
                archived_at = $9,
                updated_at = $10
            WHERE id = $1 AND household_id = $2
            "#,
            item.id().into_uuid(),
            item.household_id().into_uuid(),
            item.category_id().map(CategoryId::into_uuid),
            item.name().as_str(),
            item.name().normalized(),
            i64::from(item.current_stock()),
            i64::from(item.reorder_threshold()),
            item.priority().as_str(),
            item.archived_at(),
            item.updated_at(),
        )
        .execute(&self.pool)
        .await
        .map_err(map_write_inventory_item_error)?;

        if result.rows_affected() == 0 {
            return Err(InventoryItemRepositoryError::ItemNotFound);
        }

        Ok(())
    }
}

struct InventoryItemRow {
    id: Uuid,
    household_id: Uuid,
    category_id: Option<Uuid>,
    name: String,
    current_stock: i64,
    reorder_threshold: i64,
    priority: String,
    archived_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<InventoryItemRow> for InventoryItem {
    type Error = InventoryItemRepositoryError;

    fn try_from(value: InventoryItemRow) -> Result<Self, Self::Error> {
        let category_id = value.category_id.map(CategoryId::from_uuid);

        let name = InventoryItemName::parse(&value.name)
            .map_err(|_| InventoryItemRepositoryError::InvalidStoredData)?;

        let current_stock = u32::try_from(value.current_stock)
            .map_err(|_| InventoryItemRepositoryError::InvalidStoredData)?;

        let reorder_threshold = u32::try_from(value.reorder_threshold)
            .map_err(|_| InventoryItemRepositoryError::InvalidStoredData)?;

        let priority = InventoryPriority::parse(&value.priority)
            .map_err(|_| InventoryItemRepositoryError::InvalidStoredData)?;

        Ok(InventoryItem::new_with_archived_at(
            InventoryItemId::from_uuid(value.id),
            HouseholdId::from_uuid(value.household_id),
            category_id,
            name,
            current_stock,
            reorder_threshold,
            priority,
            value.archived_at,
            value.created_at,
            value.updated_at,
        ))
    }
}

const INVENTORY_ITEMS_PKEY: &str = "inventory_items_pkey";
const INVENTORY_ITEMS_ACTIVE_NAME_UNIQUE_IDX: &str = "inventory_items_active_name_unique_idx";
const INVENTORY_ITEMS_CATEGORY_FK: &str = "inventory_items_category_fk";

fn map_write_inventory_item_error(error: sqlx::Error) -> InventoryItemRepositoryError {
    if let Some(database_error) = error.as_database_error() {
        match database_error.constraint() {
            Some(INVENTORY_ITEMS_PKEY) | Some(INVENTORY_ITEMS_ACTIVE_NAME_UNIQUE_IDX) => {
                return InventoryItemRepositoryError::ItemAlreadyExists;
            }
            Some(INVENTORY_ITEMS_CATEGORY_FK) => {
                return InventoryItemRepositoryError::CategoryNotFound;
            }
            _ => {}
        }
    }

    map_sqlx_error(error).into()
}
