use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    modules::{
        households::domain::HouseholdId,
        inventory::{
            domain::{Category, CategoryId, CategoryName},
            ports::{CategoryRepository, CategoryRepositoryError},
        },
    },
    shared::db::map_sqlx_error,
};

pub struct PostgresCategoryRepository {
    pool: PgPool,
}

impl PostgresCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for PostgresCategoryRepository {
    async fn insert(&self, category: &Category) -> Result<(), CategoryRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO categories (
                id,
                household_id,
                name,
                normalized_name,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            category.id().into_uuid(),
            category.household_id().into_uuid(),
            category.name().as_str(),
            category.name().normalized(),
            category.created_at(),
            category.updated_at(),
        )
        .execute(&self.pool)
        .await
        .map_err(map_write_category_error)?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &CategoryId,
        household_id: &HouseholdId,
    ) -> Result<Option<Category>, CategoryRepositoryError> {
        let row = sqlx::query_as!(
            CategoryRow,
            r#"
            SELECT
                id,
                household_id,
                name,
                created_at,
                updated_at
            FROM categories
            WHERE id = $1 AND household_id = $2
            "#,
            id.into_uuid(),
            household_id.into_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        row.map(Category::try_from).transpose()
    }

    async fn find_for_household(
        &self,
        household_id: &HouseholdId,
    ) -> Result<Vec<Category>, CategoryRepositoryError> {
        let rows = sqlx::query_as!(
            CategoryRow,
            r#"
            SELECT
                id,
                household_id,
                name,
                created_at,
                updated_at
            FROM categories
            WHERE household_id = $1
            "#,
            household_id.into_uuid(),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        rows.into_iter().map(Category::try_from).collect()
    }

    async fn find_by_name(
        &self,
        household_id: &HouseholdId,
        name: &CategoryName,
    ) -> Result<Option<Category>, CategoryRepositoryError> {
        let row = sqlx::query_as!(
            CategoryRow,
            r#"
            SELECT
                id,
                household_id,
                name,
                created_at,
                updated_at
            FROM categories
            WHERE household_id = $1 AND normalized_name = $2
            "#,
            household_id.into_uuid(),
            name.normalized(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        row.map(Category::try_from).transpose()
    }

    async fn update(&self, category: &Category) -> Result<(), CategoryRepositoryError> {
        let result = sqlx::query!(
            r#"
            UPDATE categories
            SET
                name = $3,
                normalized_name = $4,
                updated_at = $5
            WHERE id = $1 AND household_id = $2
            "#,
            category.id().into_uuid(),
            category.household_id().into_uuid(),
            category.name().as_str(),
            category.name().normalized(),
            category.updated_at(),
        )
        .execute(&self.pool)
        .await
        .map_err(map_write_category_error)?;

        if result.rows_affected() == 0 {
            return Err(CategoryRepositoryError::CategoryNotFound);
        }

        Ok(())
    }

    async fn delete(
        &self,
        category_id: &CategoryId,
        household_id: &HouseholdId,
    ) -> Result<(), CategoryRepositoryError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM categories
            WHERE id = $1 AND household_id = $2
            "#,
            category_id.into_uuid(),
            household_id.into_uuid(),
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        if result.rows_affected() == 0 {
            return Err(CategoryRepositoryError::CategoryNotFound);
        }

        Ok(())
    }
}

const CATEGORIES_PKEY: &str = "categories_pkey";
const CATEGORY_HOUSEHOLD_NAME_UNIQUE: &str = "categories_household_name_unique";

fn map_write_category_error(error: sqlx::Error) -> CategoryRepositoryError {
    if let Some(database_error) = error.as_database_error() {
        match database_error.constraint() {
            Some(CATEGORIES_PKEY) | Some(CATEGORY_HOUSEHOLD_NAME_UNIQUE) => {
                return CategoryRepositoryError::CategoryAlreadyExists;
            }

            _ => {}
        }
    }

    map_sqlx_error(error).into()
}

struct CategoryRow {
    id: Uuid,
    household_id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<CategoryRow> for Category {
    type Error = CategoryRepositoryError;

    fn try_from(value: CategoryRow) -> Result<Self, Self::Error> {
        let name = CategoryName::parse(&value.name)
            .map_err(|_| CategoryRepositoryError::InvalidStoredData)?;

        Ok(Category::new(
            CategoryId::from_uuid(value.id),
            HouseholdId::from_uuid(value.household_id),
            name,
            value.created_at,
            value.updated_at,
        ))
    }
}
