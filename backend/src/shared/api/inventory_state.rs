use std::sync::Arc;

use crate::modules::inventory::application::CreateInventoryItemService;

#[derive(Clone)]
pub struct InventoryItemState {
    pub crate_inventory_item: Arc<CreateInventoryItemService>,
}
