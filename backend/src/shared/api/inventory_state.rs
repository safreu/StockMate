use std::sync::Arc;

use crate::modules::inventory::application::{
    ArchiveInventoryItemService, CreateCategoryService, CreateInventoryItemService,
    DeleteCategoryService, GetInventoryItemService, ListCategoriesService,
    ListInventoryItemsService, RestoreInventoryItemService, UpdateInventoryItemService,
};

#[derive(Clone)]
pub struct InventoryItemState {
    pub create_inventory_item: Arc<CreateInventoryItemService>,
    pub create_category: Arc<CreateCategoryService>,
    pub list_categories: Arc<ListCategoriesService>,
    pub delete_category: Arc<DeleteCategoryService>,
    pub list_inventory_items: Arc<ListInventoryItemsService>,
    pub get_inventory_item: Arc<GetInventoryItemService>,
    pub update_inventory_item: Arc<UpdateInventoryItemService>,
    pub archive_inventory_item: Arc<ArchiveInventoryItemService>,
    pub restore_inventory_item: Arc<RestoreInventoryItemService>,
}
