use crate::{
    modules::inventory::application::{
        CreateCategoryError, CreateInventoryItemError, DeleteCategoryError, GetInventoryItemError,
        ListCategoriesError, ListInventoryItemsError,
    },
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

impl From<ListCategoriesError> for ApiError {
    fn from(error: ListCategoriesError) -> Self {
        match error {
            ListCategoriesError::Forbidden => ApiError::forbidden(
                "household_access_forbidden",
                "You do not have permissions to modify this household",
            ),
            ListCategoriesError::HouseholdNotFound => {
                ApiError::not_found("household_not_found", "The household was not found")
            }
            ListCategoriesError::Internal(_) => ApiError::internal_error(),
        }
    }
}

impl From<DeleteCategoryError> for ApiError {
    fn from(error: DeleteCategoryError) -> Self {
        match error {
            DeleteCategoryError::CategoryNotFound => {
                ApiError::not_found("category_not_found", "The category was not found")
            }
            DeleteCategoryError::Forbidden => ApiError::forbidden(
                "household_access_forbidden",
                "You do not have permissions to modify this household",
            ),
            DeleteCategoryError::HouseholdNotFound => {
                ApiError::not_found("household_not_found", "The household was not found")
            }
            DeleteCategoryError::Internal(_) => ApiError::internal_error(),
        }
    }
}

impl From<ListInventoryItemsError> for ApiError {
    fn from(error: ListInventoryItemsError) -> Self {
        match error {
            ListInventoryItemsError::Forbidden => ApiError::forbidden(
                "household_access_forbidden",
                "You do not have permissions to modify this household",
            ),
            ListInventoryItemsError::HouseholdNotFound => {
                ApiError::not_found("household_not_found", "The household was not found")
            }
            ListInventoryItemsError::Internal(_) => ApiError::internal_error(),
        }
    }
}

impl From<GetInventoryItemError> for ApiError {
    fn from(error: GetInventoryItemError) -> Self {
        match error {
            GetInventoryItemError::Forbidden => ApiError::forbidden(
                "household_access_forbidden",
                "You do not have permissions to modify this household",
            ),
            GetInventoryItemError::HouseholdNotFound => {
                ApiError::not_found("household_not_found", "The household was not found")
            }
            GetInventoryItemError::ItemNotFound => ApiError::not_found(
                "inventory_item_not_found",
                "The inventory item was not found",
            ),
            GetInventoryItemError::Internal(_) => ApiError::internal_error(),
        }
    }
}
