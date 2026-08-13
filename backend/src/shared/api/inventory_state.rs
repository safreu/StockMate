use std::sync::Arc;

use crate::modules::inventory::application::{
    CreateCategoryService, CreateInventoryItemService, DeleteCategoryService,
    ListCategoriesService, ListInventoryItemsService,
};

#[derive(Clone)]
pub struct InventoryItemState {
    pub create_inventory_item: Arc<CreateInventoryItemService>,
    pub create_category: Arc<CreateCategoryService>,
    pub list_categories: Arc<ListCategoriesService>,
    pub delete_category: Arc<DeleteCategoryService>,
    pub list_inventory_items: Arc<ListInventoryItemsService>,
}
