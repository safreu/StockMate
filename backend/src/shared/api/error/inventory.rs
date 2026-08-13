use crate::{
    modules::inventory::application::{CreateCategoryError, CreateInventoryItemError},
    shared::api::ApiError,
};

impl From<CreateInventoryItemError> for ApiError {
    fn from(error: CreateInventoryItemError) -> Self {
        match error {
            CreateInventoryItemError::CategoryNotFound => {
                ApiError::not_found("category_not_found", "The category was not found")
            }
            CreateInventoryItemError::Forbidden => ApiError::forbidden(
                "household_access_forbidden",
                "You do not have permissions to modify this household",
            ),
            CreateInventoryItemError::HouseholdNotFound => {
                ApiError::not_found("household_not_found", "The household was not found")
            }
            CreateInventoryItemError::Internal(_) => ApiError::internal_error(),
            CreateInventoryItemError::InvalidName => ApiError::bad_request(
                "invalid_inventory_item_name",
                "The inventory item name is invalid",
            ),
            CreateInventoryItemError::ItemAlreadyExists => ApiError::conflict(
                "inventory_item_already_exists",
                "An active inventory item with this name already exists",
            ),
        }
    }
}

impl From<CreateCategoryError> for ApiError {
    fn from(error: CreateCategoryError) -> Self {
        match error {
            CreateCategoryError::CategoryAlreadyExists => ApiError::conflict(
                "category_already_exists",
                "A category with this name already exists",
            ),
            CreateCategoryError::Forbidden => ApiError::forbidden(
                "household_access_forbidden",
                "You do not have permissions to modify this household",
            ),
            CreateCategoryError::HouseholdNotFound => {
                ApiError::not_found("household_not_found", "The household was not found")
            }
            CreateCategoryError::InvalidName => {
                ApiError::bad_request("invalid_category_name", "The category name is invalid")
            }
            CreateCategoryError::Internal(_) => ApiError::internal_error(),
        }
    }
}
