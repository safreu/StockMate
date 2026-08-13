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
            api::dto::{
                CreateCategoryRequest, CreateCategoryResponse, CreateInventoryItemRequest,
                CreateInventoryItemResponse, ListCategoriesResponse,
            },
            application::{
                CreateCategoryCommand, CreateInventoryItemCommand, DeleteCategoryCommand,
                ListCategoriesCommand,
            },
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
        .create_inventory_item
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

pub async fn create_category(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(household_id): Path<Uuid>,
    Json(request): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<CreateCategoryResponse>), ApiError> {
    let command = CreateCategoryCommand {
        requester_id: current_user.user_id(),
        household_id: HouseholdId::from_uuid(household_id),
        name: request.name,
    };

    let category_id = state
        .inventory
        .create_category
        .execute(command)
        .await
        .map_err(ApiError::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateCategoryResponse {
            id: category_id.into_uuid(),
        }),
    ))
}

pub async fn list_categories(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(household_id): Path<Uuid>,
) -> Result<Json<Vec<ListCategoriesResponse>>, ApiError> {
    let command = ListCategoriesCommand {
        requester_id: current_user.user_id(),
        household_id: HouseholdId::from_uuid(household_id),
    };

    let categories = state
        .inventory
        .list_categories
        .execute(command)
        .await
        .map_err(ApiError::from)?;

    let response = categories
        .into_iter()
        .map(|category| ListCategoriesResponse {
            id: category.id().into_uuid(),
            name: category.name().as_str().to_string(),
        })
        .collect();

    Ok(Json(response))
}

pub async fn delete_category(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path((household_id, category_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    let command = DeleteCategoryCommand {
        requester_id: current_user.user_id(),
        household_id: HouseholdId::from_uuid(household_id),
        category_id: CategoryId::from_uuid(category_id),
    };

    state
        .inventory
        .delete_category
        .execute(command)
        .await
        .map_err(ApiError::from)?;

    Ok(StatusCode::NO_CONTENT)
}
