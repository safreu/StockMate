use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
