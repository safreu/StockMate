use chrono::{DateTime, Utc};

use crate::modules::{
    households::domain::HouseholdId,
    inventory::domain::{CategoryId, InventoryItemId, InventoryItemName, InventoryPriority},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryItem {
    id: InventoryItemId,
    household_id: HouseholdId,
    category_id: Option<CategoryId>,
    name: InventoryItemName,
    current_stock: u32,
    reorder_threshold: u32,
    priority: InventoryPriority,
    archived_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl InventoryItem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: InventoryItemId,
        household_id: HouseholdId,
        category_id: Option<CategoryId>,
        name: InventoryItemName,
        current_stock: u32,
        reorder_threshold: u32,
        priority: InventoryPriority,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            household_id,
            category_id,
            name,
            current_stock,
            reorder_threshold,
            priority,
            archived_at: None,
            created_at,
            updated_at,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_with_archived_at(
        id: InventoryItemId,
        household_id: HouseholdId,
        category_id: Option<CategoryId>,
        name: InventoryItemName,
        current_stock: u32,
        reorder_threshold: u32,
        priority: InventoryPriority,
        archived_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            household_id,
            category_id,
            name,
            current_stock,
            reorder_threshold,
            priority,
            archived_at,
            created_at,
            updated_at,
        }
    }

    fn ensure_active(&self) -> Result<(), InventoryItemError> {
        if self.archived_at().is_some() {
            return Err(InventoryItemError::Archived);
        }

        Ok(())
    }

    pub fn id(&self) -> InventoryItemId {
        self.id
    }

    pub fn household_id(&self) -> HouseholdId {
        self.household_id
    }

    pub fn category_id(&self) -> Option<CategoryId> {
        self.category_id
    }

    pub fn name(&self) -> &InventoryItemName {
        &self.name
    }

    pub fn current_stock(&self) -> u32 {
        self.current_stock
    }

    pub fn reorder_threshold(&self) -> u32 {
        self.reorder_threshold
    }

    pub fn priority(&self) -> InventoryPriority {
        self.priority
    }

    pub fn archived_at(&self) -> Option<DateTime<Utc>> {
        self.archived_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn shopping_quantity(&self) -> Result<u32, InventoryItemError> {
        self.ensure_active()?;
        Ok(calculate_shopping_quantity(
            self.current_stock,
            self.reorder_threshold,
        ))
    }

    pub fn increase(&mut self, amount: u32, now: DateTime<Utc>) -> Result<(), InventoryItemError> {
        self.ensure_active()?;

        self.current_stock = self
            .current_stock
            .checked_add(amount)
            .ok_or(InventoryItemError::StockOverflow)?;

        self.updated_at = now;

        Ok(())
    }

    pub fn decrease(&mut self, amount: u32, now: DateTime<Utc>) -> Result<(), InventoryItemError> {
        self.ensure_active()?;

        if amount > self.current_stock {
            return Err(InventoryItemError::InsufficientStock);
        }

        self.current_stock -= amount;
        self.updated_at = now;

        Ok(())
    }

    pub fn set_stock(&mut self, stock: u32, now: DateTime<Utc>) -> Result<(), InventoryItemError> {
        self.ensure_active()?;

        self.current_stock = stock;
        self.updated_at = now;

        Ok(())
    }

    pub fn archive(&mut self, now: DateTime<Utc>) -> Result<(), InventoryItemError> {
        if self.archived_at().is_some() {
            return Err(InventoryItemError::AlreadyArchived);
        }

        self.archived_at = Some(now);
        self.updated_at = now;

        Ok(())
    }

    pub fn restore(&mut self, now: DateTime<Utc>) -> Result<(), InventoryItemError> {
        if self.archived_at().is_none() {
            return Err(InventoryItemError::NotArchived);
        }

        self.archived_at = None;
        self.updated_at = now;

        Ok(())
    }

    pub fn rename(
        &mut self,
        name: InventoryItemName,
        now: DateTime<Utc>,
    ) -> Result<(), InventoryItemError> {
        self.ensure_active()?;

        self.name = name;
        self.updated_at = now;

        Ok(())
    }

    pub fn change_category(
        &mut self,
        category_id: Option<CategoryId>,
        now: DateTime<Utc>,
    ) -> Result<(), InventoryItemError> {
        self.ensure_active()?;

        self.category_id = category_id;
        self.updated_at = now;

        Ok(())
    }

    pub fn set_reorder_threshold(
        &mut self,
        threshold: u32,
        now: DateTime<Utc>,
    ) -> Result<(), InventoryItemError> {
        self.ensure_active()?;

        self.reorder_threshold = threshold;
        self.updated_at = now;

        Ok(())
    }

    pub fn set_priority(
        &mut self,
        priority: InventoryPriority,
        now: DateTime<Utc>,
    ) -> Result<(), InventoryItemError> {
        self.ensure_active()?;

        self.priority = priority;
        self.updated_at = now;

        Ok(())
    }
}

pub fn calculate_shopping_quantity(current_stock: u32, reorder_threshold: u32) -> u32 {
    if current_stock <= reorder_threshold {
        reorder_threshold - current_stock + 1
    } else {
        0
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum InventoryItemError {
    #[error("Stock value overflow")]
    StockOverflow,
    #[error("Insufficient stock")]
    InsufficientStock,
    #[error("Inventory item is archived")]
    Archived,
    #[error("Inventory item is already archived")]
    AlreadyArchived,
    #[error("Inventory item is not archived")]
    NotArchived,
}

#[cfg(test)]
mod tests {

    use super::*;

    fn test_item(current_stock: u32, reorder_threshold: u32) -> InventoryItem {
        InventoryItem::new(
            InventoryItemId::new(),
            HouseholdId::new(),
            Some(CategoryId::new()),
            InventoryItemName::parse("Tofu").expect("Inventory item name should be valid"),
            current_stock,
            reorder_threshold,
            InventoryPriority::Default,
            Utc::now(),
            Utc::now(),
        )
    }

    #[test]
    fn shopping_quantity_is_zero_above_threshold() {
        let item = test_item(6, 5);

        assert_eq!(item.shopping_quantity(), Ok(0))
    }

    #[test]
    fn shopping_quantity_is_one_at_threshold() {
        let item = test_item(5, 5);

        assert_eq!(item.shopping_quantity(), Ok(1))
    }

    #[test]
    fn shopping_quantity_returns_correct_value_below_threshold() {
        let item = test_item(2, 5);

        assert_eq!(item.shopping_quantity(), Ok(4))
    }

    #[test]
    fn increase_increases_stock_and_updates_updated_at() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        item.increase(4, now)
            .expect("Stock increase should succeed");

        assert_eq!(item.current_stock(), 10);
        assert_eq!(item.updated_at(), now)
    }

    #[test]
    fn increase_returns_stock_overflow_on_overflow() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        let result = item.increase(u32::MAX, now);

        assert_eq!(result, Err(InventoryItemError::StockOverflow))
    }

    #[test]
    fn decrease_decreases_stock_and_updates_updated_at() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        item.decrease(4, now)
            .expect("Stock decrease should succeed");

        assert_eq!(item.current_stock(), 2);
        assert_eq!(item.updated_at(), now)
    }

    #[test]
    fn decrease_rejects_going_below_zero_and_leaves_state_unchanged() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        let result = item.decrease(10, now);

        assert_eq!(result, Err(InventoryItemError::InsufficientStock));
    }

    #[test]
    fn set_stock_overwrites_stock() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        item.set_stock(1, now)
            .expect("Setting stock should succeed");

        assert_eq!(item.current_stock(), 1);
        assert_eq!(item.updated_at(), now);
    }

    #[test]
    fn rename_changes_name() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        let new_name = InventoryItemName::parse("Valid name").expect("Name should be valid");

        item.rename(new_name.clone(), now)
            .expect("Item renaming should succeed");

        assert_eq!(item.name(), &new_name);
        assert_eq!(item.updated_at(), now);
    }

    #[test]
    fn change_category_sets_category() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        let category_id = Some(CategoryId::new());

        item.change_category(category_id, now)
            .expect("Category change should be valid");

        assert_eq!(item.category_id(), category_id);
        assert_eq!(item.updated_at(), now);
    }

    #[test]
    fn change_category_can_set_none() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        let category_id = None;

        item.change_category(category_id, now)
            .expect("Category change should be valid");

        assert!(item.category_id().is_none());
        assert_eq!(item.updated_at(), now);
    }

    #[test]
    fn set_reorder_threshold_sets_the_threshold() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        item.set_reorder_threshold(2, now)
            .expect("Set reorder threshold should be valid");

        assert_eq!(item.reorder_threshold(), 2);
        assert_eq!(item.updated_at(), now);
    }

    #[test]
    fn set_priority_sets_priority() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        item.set_priority(InventoryPriority::High, now)
            .expect("Setting priority should succeed");

        assert_eq!(item.priority(), InventoryPriority::High);
        assert_eq!(item.updated_at(), now);
    }

    #[test]
    fn archive_set_archived_at() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        item.archive(now).expect("Archiving should succeed");

        assert_eq!(item.archived_at(), Some(now));
        assert_eq!(item.updated_at(), now);
    }

    #[test]
    fn archiving_twice_returns_already_archived() {
        let mut item = test_item(6, 5);

        item.archive(Utc::now()).expect("Archiving should succeed");

        let now = Utc::now();

        let result = item.archive(now);

        assert_eq!(result, Err(InventoryItemError::AlreadyArchived));
    }

    #[test]
    fn restore_clears_archived_at() {
        let mut item = test_item(6, 5);

        item.archive(Utc::now()).expect("Archiving should succeed");

        let now = Utc::now();

        item.restore(now).expect("Restoring should succeed");

        assert!(item.archived_at().is_none());
        assert_eq!(item.updated_at(), now)
    }

    #[test]
    fn restoring_active_item_returns_not_archived() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        let result = item.restore(now);

        assert_eq!(result, Err(InventoryItemError::NotArchived))
    }

    #[test]
    fn operations_on_archived_item_returns_archived() {
        let mut item = test_item(6, 5);

        let now = Utc::now();

        item.archive(now).expect("Archiving should succeed");

        assert_eq!(item.increase(5, now), Err(InventoryItemError::Archived));
        assert_eq!(item.decrease(5, now), Err(InventoryItemError::Archived));
        assert_eq!(item.set_stock(5, now), Err(InventoryItemError::Archived));
        assert_eq!(item.shopping_quantity(), Err(InventoryItemError::Archived));
    }
}
