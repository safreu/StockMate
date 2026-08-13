use std::sync::Arc;

use crate::modules::inventory::application::{CreateCategoryService, CreateInventoryItemService};

#[derive(Clone)]
pub struct InventoryItemState {
    pub create_inventory_item: Arc<CreateInventoryItemService>,
    pub create_category: Arc<CreateCategoryService>,
}
