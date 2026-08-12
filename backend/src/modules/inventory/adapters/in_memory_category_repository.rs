use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::modules::{
    households::domain::HouseholdId,
    inventory::{
        domain::{Category, CategoryId, CategoryName},
        ports::{CategoryRepository, CategoryRepositoryError},
    },
};

struct InMemoryCategoryState {
    categories: HashMap<CategoryId, Category>,
}

pub struct InMemoryCategoryRepository {
    state: RwLock<InMemoryCategoryState>,
}

impl Default for InMemoryCategoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryCategoryRepository {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(InMemoryCategoryState {
                categories: HashMap::new(),
            }),
        }
    }
}

#[async_trait]
impl CategoryRepository for InMemoryCategoryRepository {
    async fn insert(&self, category: &Category) -> Result<(), CategoryRepositoryError> {
        let mut state = self.state.write().await;

        if state.categories.contains_key(&category.id()) {
            return Err(CategoryRepositoryError::CategoryAlreadyExists);
        }

        let normalized_name = category.name().normalized();

        let existing = state.categories.values().any(|existing| {
            existing.household_id() == category.household_id()
                && existing.name().normalized() == normalized_name
        });

        if existing {
            return Err(CategoryRepositoryError::CategoryAlreadyExists);
        }

        state.categories.insert(category.id(), category.clone());

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &CategoryId,
        household_id: &HouseholdId,
    ) -> Result<Option<Category>, CategoryRepositoryError> {
        let state = self.state.read().await;

        Ok(state
            .categories
            .get(id)
            .filter(|category| category.household_id() == *household_id)
            .cloned())
    }

    async fn find_for_household(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<Category>, CategoryRepositoryError> {
        let state = self.state.read().await;

        Ok(state
            .categories
            .values()
            .filter(|category| category.household_id() == *household_id)
            .cloned()
            .collect())
    }

    async fn find_by_name(
        &self,
        household_id: &HouseholdId,
        name: &CategoryName,
    ) -> Result<Option<Category>, CategoryRepositoryError> {
        let state = self.state.read().await;

        let normalized_name = name.normalized();

        Ok(state
            .categories
            .values()
            .find(|category| {
                category.household_id() == *household_id
                    && category.name().normalized() == normalized_name
            })
            .cloned())
    }

    async fn update(&self, category: &Category) -> Result<(), CategoryRepositoryError> {
        let mut state = self.state.write().await;

        let stored = state
            .categories
            .get(&category.id())
            .ok_or(CategoryRepositoryError::CategoryNotFound)?;

        if stored.household_id() != category.household_id() {
            return Err(CategoryRepositoryError::CategoryNotFound);
        }

        let normalized_name = category.name().normalized();

        let existing = state.categories.values().any(|existing| {
            existing.id() != category.id()
                && existing.household_id() == category.household_id()
                && existing.name().normalized() == normalized_name
        });

        if existing {
            return Err(CategoryRepositoryError::CategoryAlreadyExists);
        }

        state.categories.insert(category.id(), category.clone());

        Ok(())
    }

    async fn delete(
        &self,
        category_id: &CategoryId,
        household_id: &HouseholdId,
    ) -> Result<(), CategoryRepositoryError> {
        let mut state = self.state.write().await;

        state
            .categories
            .get(category_id)
            .filter(|category| category.household_id() == *household_id)
            .ok_or(CategoryRepositoryError::CategoryNotFound)?;

        state.categories.remove(category_id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    fn test_category(household_id: HouseholdId, name: &str) -> Category {
        Category::new(
            CategoryId::new(),
            household_id,
            CategoryName::parse(name).expect("Category name should be valid"),
            Utc::now(),
            Utc::now(),
        )
    }

    #[tokio::test]
    async fn category_can_be_inserted_and_loaded() {
        let repository = InMemoryCategoryRepository::default();

        let category = test_category(HouseholdId::new(), "Fruit");

        let insertion = repository.insert(&category).await;

        assert!(insertion.is_ok());

        let stored = repository
            .find_by_id(&category.id(), &category.household_id())
            .await
            .expect("Category lookup should succeed");

        assert_eq!(stored, Some(category))
    }

    #[tokio::test]
    async fn duplicate_normalized_name_in_same_household_gets_rejected() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");
        let duplicate_category = test_category(household_id, "fruit");

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");
        let insertion = repository.insert(&duplicate_category).await;

        assert_eq!(
            insertion,
            Err(CategoryRepositoryError::CategoryAlreadyExists)
        )
    }

    #[tokio::test]
    async fn same_normalized_name_in_different_households_is_allowed() {
        let repository = InMemoryCategoryRepository::default();

        let category = test_category(HouseholdId::new(), "Fruit");
        let duplicate_category = test_category(HouseholdId::new(), "fruit");

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");
        let insertion = repository.insert(&duplicate_category).await;

        assert!(insertion.is_ok())
    }

    #[tokio::test]
    async fn category_can_be_found_by_normalized_name() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");

        let stored = repository
            .find_by_name(
                &household_id,
                &CategoryName::parse("fruit").expect("Category name should be valid"),
            )
            .await
            .expect("Category lookup should succeed");

        assert_eq!(stored, Some(category));
    }

    #[tokio::test]
    async fn find_for_household_returns_only_categories_for_that_household() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");
        let another_category = test_category(HouseholdId::new(), "Vegetables");

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");

        repository
            .insert(&another_category)
            .await
            .expect("Category insertion should succeed");

        let stored = repository
            .find_for_household(&household_id)
            .await
            .expect("Category lookup should succeed");

        assert_eq!(stored.len(), 1);
        assert!(stored.contains(&category));
        assert!(!stored.contains(&another_category));
    }

    #[tokio::test]
    async fn existing_category_can_be_updated() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");

        let updated_category = Category::new(
            category.id(),
            household_id,
            CategoryName::parse("Vegetables").expect("Category name should be valid"),
            category.created_at(),
            Utc::now(),
        );

        repository
            .update(&updated_category)
            .await
            .expect("Category update should succeed");

        let stored = repository
            .find_by_id(&updated_category.id(), &household_id)
            .await
            .expect("Category lookup should succeed");

        assert_eq!(stored, Some(updated_category));
        assert_ne!(stored, Some(category));
    }

    #[tokio::test]
    async fn updating_category_to_duplicating_name_is_rejected() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");
        let another_category = test_category(household_id, "Vegetables");

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");

        repository
            .insert(&another_category)
            .await
            .expect("Category insertion should succeed");

        let updated_category = Category::new(
            category.id(),
            household_id,
            CategoryName::parse("vegetables").expect("Category name should be valid"),
            category.created_at(),
            Utc::now(),
        );

        let update = repository.update(&updated_category).await;

        assert_eq!(update, Err(CategoryRepositoryError::CategoryAlreadyExists))
    }

    #[tokio::test]
    async fn updating_category_with_different_household_is_rejected() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");

        let updated_category = Category::new(
            category.id(),
            HouseholdId::new(),
            CategoryName::parse("Vegetables").expect("Category name should be valid"),
            category.created_at(),
            Utc::now(),
        );

        let update = repository.update(&updated_category).await;

        assert_eq!(update, Err(CategoryRepositoryError::CategoryNotFound))
    }

    #[tokio::test]
    async fn existing_category_can_be_deleted() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");

        repository
            .delete(&category.id(), &household_id)
            .await
            .expect("Category deletion should succeed");

        let stored = repository
            .find_by_id(&category.id(), &category.household_id())
            .await
            .expect("Category lookup should succeed");

        assert!(stored.is_none())
    }

    #[tokio::test]
    async fn deleting_category_from_different_household_returns_not_found() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");

        let deletion = repository.delete(&category.id(), &HouseholdId::new()).await;

        assert_eq!(deletion, Err(CategoryRepositoryError::CategoryNotFound))
    }

    #[tokio::test]
    async fn deleting_unknown_category_returns_not_found() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");

        let deletion = repository.delete(&CategoryId::new(), &household_id).await;

        assert_eq!(deletion, Err(CategoryRepositoryError::CategoryNotFound))
    }

    #[tokio::test]
    async fn duplicate_category_id_is_rejected() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let category = test_category(household_id, "Fruit");
        let duplicate_category = Category::new(
            category.id(),
            household_id,
            CategoryName::parse("Vegetables").expect("Category name should be valid"),
            Utc::now(),
            Utc::now(),
        );

        repository
            .insert(&category)
            .await
            .expect("Category insertion should succeed");
        let insertion = repository.insert(&duplicate_category).await;

        assert_eq!(
            insertion,
            Err(CategoryRepositoryError::CategoryAlreadyExists)
        )
    }

    #[tokio::test]
    async fn updating_unknown_category_returns_not_found() {
        let repository = InMemoryCategoryRepository::default();

        let household_id = HouseholdId::new();

        let updated_category = Category::new(
            CategoryId::new(),
            household_id,
            CategoryName::parse("Vegetables").expect("Category name should be valid"),
            Utc::now(),
            Utc::now(),
        );

        let update = repository.update(&updated_category).await;

        assert_eq!(update, Err(CategoryRepositoryError::CategoryNotFound))
    }
}
