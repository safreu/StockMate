use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::modules::{
    households::domain::HouseholdId,
    inventory::{
        domain::{InventoryItem, InventoryItemId, InventoryItemName},
        ports::{InventoryItemRepository, InventoryItemRepositoryError},
    },
};

struct InMemoryInventoryItemState {
    items: HashMap<InventoryItemId, InventoryItem>,
}

pub struct InMemoryInventoryItemRepository {
    state: RwLock<InMemoryInventoryItemState>,
}

impl InMemoryInventoryItemRepository {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(InMemoryInventoryItemState {
                items: HashMap::new(),
            }),
        }
    }
}

impl Default for InMemoryInventoryItemRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InventoryItemRepository for InMemoryInventoryItemRepository {
    async fn insert(&self, item: &InventoryItem) -> Result<(), InventoryItemRepositoryError> {
        let mut state = self.state.write().await;

        if state.items.contains_key(&item.id()) {
            return Err(InventoryItemRepositoryError::ItemAlreadyExists);
        }

        let normalized_name = item.name().normalized();

        let active_existing = state.items.values().any(|existing| {
            existing.household_id() == item.household_id()
                && existing.archived_at().is_none()
                && item.archived_at().is_none()
                && existing.name().normalized() == normalized_name
        });

        if active_existing {
            return Err(InventoryItemRepositoryError::ItemAlreadyExists);
        }

        state.items.insert(item.id(), item.clone());

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &InventoryItemId,
        household_id: &HouseholdId,
    ) -> Result<Option<InventoryItem>, InventoryItemRepositoryError> {
        let state = self.state.read().await;

        Ok(state
            .items
            .get(id)
            .filter(|item| item.household_id() == *household_id)
            .cloned())
    }

    async fn find_active_by_name(
        &self,
        household_id: &HouseholdId,
        name: &InventoryItemName,
    ) -> Result<Option<InventoryItem>, InventoryItemRepositoryError> {
        let state = self.state.read().await;

        let normalized_name = name.normalized();

        Ok(state
            .items
            .values()
            .find(|item| {
                item.household_id() == *household_id
                    && item.archived_at().is_none()
                    && item.name().normalized() == normalized_name
            })
            .cloned())
    }

    async fn find_active_for_household(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<InventoryItem>, InventoryItemRepositoryError> {
        let state = self.state.read().await;

        Ok(state
            .items
            .values()
            .filter(|item| item.household_id() == *household_id && item.archived_at().is_none())
            .cloned()
            .collect())
    }

    async fn find_archived_for_household(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<InventoryItem>, InventoryItemRepositoryError> {
        let state = self.state.read().await;

        Ok(state
            .items
            .values()
            .filter(|item| item.archived_at().is_some() && item.household_id() == *household_id)
            .cloned()
            .collect())
    }

    async fn update(&self, item: &InventoryItem) -> Result<(), InventoryItemRepositoryError> {
        let mut state = self.state.write().await;

        let stored = state
            .items
            .get(&item.id())
            .ok_or(InventoryItemRepositoryError::ItemNotFound)?;

        if stored.household_id() != item.household_id() {
            return Err(InventoryItemRepositoryError::ItemNotFound);
        }

        if item.archived_at().is_none() {
            let normalized_name = item.name().normalized();

            let active_existing = state.items.values().any(|existing| {
                existing.id() != item.id()
                    && existing.household_id() == item.household_id()
                    && existing.archived_at().is_none()
                    && existing.name().normalized() == normalized_name
            });

            if active_existing {
                return Err(InventoryItemRepositoryError::ItemAlreadyExists);
            }
        }

        state.items.insert(item.id(), item.clone());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::modules::inventory::domain::{
        Category, CategoryId, CategoryName, InventoryPriority,
    };

    use super::*;

    fn test_category(household_id: HouseholdId, name: &str) -> Category {
        Category::new(
            CategoryId::new(),
            household_id,
            CategoryName::parse(name).expect("Category name should be valid"),
            Utc::now(),
            Utc::now(),
        )
    }

    fn test_inventory_item(
        household_id: HouseholdId,
        category_id: Option<CategoryId>,
        name: &str,
    ) -> InventoryItem {
        InventoryItem::new(
            InventoryItemId::new(),
            household_id,
            category_id,
            InventoryItemName::parse(name).expect("Inventory item name should be valid"),
            10,
            0,
            InventoryPriority::High,
            Utc::now(),
            Utc::now(),
        )
    }

    #[tokio::test]
    async fn inventory_item_can_be_inserted_and_loaded() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");

        let item = test_inventory_item(household_id, Some(category.id()), "Apple");

        let result = repository.insert(&item).await;

        assert!(result.is_ok());

        let stored = repository
            .find_by_id(&item.id(), &household_id)
            .await
            .expect("Inventory item lookup should succeed");

        assert_eq!(stored, Some(item))
    }

    #[tokio::test]
    async fn duplicate_inventory_item_id_is_rejected() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let item = test_inventory_item(household_id, None, "Apple");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");

        let result = repository.insert(&item).await;

        assert_eq!(result, Err(InventoryItemRepositoryError::ItemAlreadyExists))
    }

    #[tokio::test]
    async fn duplicate_active_normalized_name_in_same_household_is_rejected() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let item = test_inventory_item(household_id, None, "apple");
        let another_item = test_inventory_item(household_id, None, "Apple");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");

        let result = repository.insert(&another_item).await;

        assert_eq!(result, Err(InventoryItemRepositoryError::ItemAlreadyExists))
    }

    #[tokio::test]
    async fn same_normalized_name_in_different_households_is_allowed() {
        let repository = InMemoryInventoryItemRepository::new();

        let item = test_inventory_item(HouseholdId::new(), None, "apple");
        let another_item = test_inventory_item(HouseholdId::new(), None, "Apple");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");

        let result = repository.insert(&another_item).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn archived_item_does_not_block_reusing_its_name() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let mut item = test_inventory_item(household_id, None, "apple");
        let another_item = test_inventory_item(household_id, None, "Apple");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");

        item.archive(Utc::now())
            .expect("Inventory item archiving should succeed");

        repository
            .update(&item)
            .await
            .expect("Inventory item updating should succeed");

        let result = repository.insert(&another_item).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn active_item_can_be_found_by_normalized_name() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let item = test_inventory_item(household_id, None, "Apple");

        repository
            .insert(&item)
            .await
            .expect("Inventory item should succeed");

        let stored = repository
            .find_active_by_name(
                &household_id,
                &InventoryItemName::parse("apple").expect("Inventory item name should be valid"),
            )
            .await
            .expect("Inventory item lookup should succeed");

        assert_eq!(stored, Some(item))
    }

    #[tokio::test]
    async fn archived_item_is_not_found_by_active_name_lookup() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let mut item = test_inventory_item(household_id, None, "Apple");

        repository
            .insert(&item)
            .await
            .expect("Inventory item should succeed");

        item.archive(Utc::now())
            .expect("Inventory item archiving should succeed");

        repository
            .update(&item)
            .await
            .expect("Inventory item updating should succeed");

        let stored = repository
            .find_active_by_name(
                &household_id,
                &InventoryItemName::parse("apple").expect("Inventory item name should be valid"),
            )
            .await
            .expect("Inventory item lookup should succeed");

        assert!(stored.is_none())
    }

    #[tokio::test]
    async fn find_active_for_household_returns_only_active_items_of_that_household() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let mut item = test_inventory_item(household_id, None, "apple");
        let another_item = test_inventory_item(household_id, None, "oranges");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");

        repository
            .insert(&another_item)
            .await
            .expect("Inventory item insertion should succeed");

        item.archive(Utc::now())
            .expect("Inventory item archiving should succeed");

        repository
            .update(&item)
            .await
            .expect("Inventory item updating should succeed");

        let stored = repository
            .find_active_for_household(&household_id)
            .await
            .expect("Inventory item lookup should succeed");

        assert_eq!(stored.len(), 1);
        assert!(stored.contains(&another_item));
        assert!(!stored.contains(&item));
    }

    #[tokio::test]
    async fn find_archived_for_household_returns_only_archived_items_of_that_household() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let mut item = test_inventory_item(household_id, None, "apple");
        let another_item = test_inventory_item(household_id, None, "oranges");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");

        repository
            .insert(&another_item)
            .await
            .expect("Inventory item insertion should succeed");

        item.archive(Utc::now())
            .expect("Inventory item archiving should succeed");

        repository
            .update(&item)
            .await
            .expect("Inventory item updating should succeed");

        let stored = repository
            .find_archived_for_household(&household_id)
            .await
            .expect("Inventory item lookup should succeed");

        assert_eq!(stored.len(), 1);
        assert!(!stored.contains(&another_item));
        assert!(stored.contains(&item));
    }

    #[tokio::test]
    async fn existing_inventory_item_can_be_updated() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let mut item = test_inventory_item(household_id, None, "apple");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");

        let now = Utc::now();

        item.set_priority(InventoryPriority::Default, now)
            .expect("Setting priority should succeed");

        repository
            .update(&item)
            .await
            .expect("Inventory item updating should succeed");

        let stored = repository
            .find_by_id(&item.id(), &household_id)
            .await
            .expect("Inventory item lookup should succeed")
            .expect("Item should exist");

        assert_eq!(stored.updated_at(), now);
        assert_eq!(stored.priority(), InventoryPriority::Default)
    }

    #[tokio::test]
    async fn updating_inventory_item_with_different_household_is_rejected() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let item = test_inventory_item(household_id, None, "apple");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");

        let updated_item = InventoryItem::new(
            item.id(),
            HouseholdId::new(),
            None,
            InventoryItemName::parse("new name").expect("Inventory item name should be valid"),
            item.current_stock(),
            item.reorder_threshold(),
            item.priority(),
            item.created_at(),
            item.updated_at(),
        );

        let result = repository.update(&updated_item).await;

        assert_eq!(result, Err(InventoryItemRepositoryError::ItemNotFound))
    }

    #[tokio::test]
    async fn updating_inventory_item_to_duplicate_active_name_is_rejected() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let item = test_inventory_item(household_id, None, "apple");
        let another_item = test_inventory_item(household_id, None, "oranges");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");
        repository
            .insert(&another_item)
            .await
            .expect("Inventory item insertion should succeed");

        let updated_item = InventoryItem::new(
            item.id(),
            household_id,
            None,
            InventoryItemName::parse("oranges").expect("Inventory item name should be valid"),
            item.current_stock(),
            item.reorder_threshold(),
            item.priority(),
            item.created_at(),
            item.updated_at(),
        );

        let result = repository.update(&updated_item).await;

        assert_eq!(result, Err(InventoryItemRepositoryError::ItemAlreadyExists))
    }

    #[tokio::test]
    async fn restoring_item_with_duplicate_active_name_is_rejected() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let mut item = test_inventory_item(household_id, None, "apple");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");

        item.archive(Utc::now())
            .expect("Inventory item archiving should succeed");

        repository
            .update(&item)
            .await
            .expect("Inventory item update should succeed");

        let another_item = test_inventory_item(household_id, None, "apple");

        repository
            .insert(&another_item)
            .await
            .expect("Inventory item insertion should succeed");

        item.restore(Utc::now())
            .expect("Inventory item restoration should succeed");

        let result = repository.update(&item).await;

        assert_eq!(result, Err(InventoryItemRepositoryError::ItemAlreadyExists))
    }

    #[tokio::test]
    async fn updating_unknown_inventory_item_returns_not_found() {
        let repository = InMemoryInventoryItemRepository::new();

        let household_id = HouseholdId::new();

        let item = test_inventory_item(household_id, None, "apple");
        let another_item = test_inventory_item(household_id, None, "apples");

        repository
            .insert(&item)
            .await
            .expect("Inventory item insertion should succeed");

        let result = repository.update(&another_item).await;

        assert_eq!(result, Err(InventoryItemRepositoryError::ItemNotFound))
    }
}
