use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    modules::{
        accounts::api::CurrentUser,
        households::domain::HouseholdId,
        inventory::{
            api::dto::{CreateInventoryItemRequest, CreateInventoryItemResponse},
            application::CreateInventoryItemCommand,
            domain::{CategoryId, InventoryPriority},
        },
    },
    shared::api::{ApiError, AppState},
};

pub async fn create_inventory_item(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(household_id): Path<Uuid>,
    Json(request): Json<CreateInventoryItemRequest>,
) -> Result<(StatusCode, Json<CreateInventoryItemResponse>), ApiError> {
    let priority = request
        .priority
        .map(|s| {
            InventoryPriority::parse(&s).map_err(|_| {
                ApiError::bad_request(
                    "invalid_inventory_priority",
                    "The inventory priority is invalid",
                )
            })
        })
        .transpose()?;

    let command = CreateInventoryItemCommand {
        requester_id: current_user.user_id(),
        household_id: HouseholdId::from_uuid(household_id),
        category_id: request.category_id.map(CategoryId::from_uuid),
        name: request.name,
        current_stock: request.current_stock,
        reorder_threshold: request.reorder_threshold,
        priority,
    };

    let item_id = state
        .inventory
        .crate_inventory_item
        .execute(command)
        .await
        .map_err(ApiError::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateInventoryItemResponse {
            id: item_id.into_uuid(),
        }),
    ))
}
