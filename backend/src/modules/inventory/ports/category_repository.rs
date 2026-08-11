use async_trait::async_trait;

use crate::{
    modules::{
        households::domain::HouseholdId,
        inventory::domain::{Category, CategoryId, CategoryName},
    },
    shared::db::PersistenceError,
};

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn insert(&self, category: &Category) -> Result<(), CategoryRepositoryError>;

    async fn find_by_id(
        &self,
        id: &CategoryId,
        household_id: &HouseholdId,
    ) -> Result<Option<Category>, CategoryRepositoryError>;

    async fn find_for_household(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<Category>, CategoryRepositoryError>;

    async fn find_by_name(
        &self,
        household_id: &HouseholdId,
        name: &CategoryName,
    ) -> Result<Option<Category>, CategoryRepositoryError>;

    async fn update(&self, category: &Category) -> Result<(), CategoryRepositoryError>;

    async fn delete(
        &self,
        category_id: &CategoryId,
        household_id: &HouseholdId,
    ) -> Result<(), CategoryRepositoryError>;
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CategoryRepositoryError {
    #[error("Category already exists")]
    CategoryAlreadyExists,
    #[error("Category not found")]
    CategoryNotFound,
    #[error("Invalid stored category data")]
    InvalidStoredData,
    #[error(transparent)]
    Persistence(#[from] PersistenceError),
}
