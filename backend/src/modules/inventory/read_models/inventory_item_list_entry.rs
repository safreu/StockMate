use crate::modules::inventory::domain::{
    CategoryId, CategoryName, InventoryItemId, InventoryItemName, InventoryPriority,
};

pub struct InventoryItemListEntry {
    pub id: InventoryItemId,
    pub name: InventoryItemName,
    pub category: Option<CategorySummary>,
    pub current_stock: u32,
    pub reorder_threshold: u32,
    pub priority: InventoryPriority,
    pub shopping_quantity: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CategorySummary {
    pub id: CategoryId,
    pub name: CategoryName,
}
