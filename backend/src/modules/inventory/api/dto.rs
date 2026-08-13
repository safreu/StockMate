use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::inventory::read_models::{CategorySummary, InventoryItemListEntry};

#[derive(Debug, Deserialize)]
pub struct CreateInventoryItemRequest {
    pub category_id: Option<Uuid>,
    pub name: String,
    pub current_stock: u32,
    pub reorder_threshold: u32,
    pub priority: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateInventoryItemResponse {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateCategoryResponse {
    pub id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ListCategoriesResponse {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct InventoryItemResponse {
    pub id: Uuid,
    pub name: String,
    pub category: Option<InventoryItemCategoryResponse>,
    pub current_stock: u32,
    pub reorder_threshold: u32,
    pub priority: String,
    pub shopping_quantity: u32,
}

impl From<InventoryItemListEntry> for InventoryItemResponse {
    fn from(value: InventoryItemListEntry) -> Self {
        Self {
            id: value.id.into_uuid(),
            name: value.name.as_str().to_owned(),
            category: value.category.map(InventoryItemCategoryResponse::from),
            current_stock: value.current_stock,
            reorder_threshold: value.reorder_threshold,
            priority: value.priority.as_str().to_owned(),
            shopping_quantity: value.shopping_quantity,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct InventoryItemCategoryResponse {
    pub id: Uuid,
    pub name: String,
}

impl From<CategorySummary> for InventoryItemCategoryResponse {
    fn from(value: CategorySummary) -> Self {
        Self {
            id: value.id.into_uuid(),
            name: value.name.as_str().to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateInventoryItemRequest {
    pub name: Option<String>,
    pub category_id: Option<Option<Uuid>>,
    pub reorder_threshold: Option<u32>,
    pub priority: Option<String>,
}
