use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    modules::{
        accounts::domain::UserId,
        households::{
            adapters::validate::validate_aggregate,
            domain::{Household, HouseholdId, HouseholdKind, HouseholdMember, HouseholdName},
            ports::{HouseholdRepository, HouseholdRepositoryError},
        },
    },
    shared::db::map_sqlx_error,
};

pub struct PostgresHouseholdRepository {
    pool: PgPool,
}

impl PostgresHouseholdRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HouseholdRepository for PostgresHouseholdRepository {
    async fn create_with_owner(
        &self,
        household: &Household,
        owner: &HouseholdMember,
    ) -> Result<(), HouseholdRepositoryError> {
        validate_aggregate(household, owner)?;

        let mut transaction = self.pool.begin().await.map_err(map_sqlx_error)?;

        sqlx::query!(
            r#"
            INSERT INTO households (
                id,
                name,
                kind,
                personal_owner_id,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            household.id().into_uuid(),
            household.name().as_str(),
            household.kind().as_str(),
            household.personal_owner_id().map(|id| id.into_uuid()),
            household.created_at(),
            household.updated_at(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(map_insert_error)?;

        sqlx::query!(
            r#"
            INSERT INTO household_members (
                household_id,
                user_id,
                role,
                created_at
            )
            VALUES ($1, $2, $3, $4)
            "#,
            owner.household_id().into_uuid(),
            owner.user_id().into_uuid(),
            owner.role().as_str(),
            owner.created_at(),
        )
        .execute(&mut *transaction)
        .await
        .map_err(map_sqlx_error)?;

        transaction.commit().await.map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &HouseholdId,
    ) -> Result<Option<Household>, HouseholdRepositoryError> {
        let row = sqlx::query_as!(
            HouseholdRow,
            r#"
            SELECT
                id,
                name,
                kind,
                personal_owner_id,
                created_at,
                updated_at
            FROM households
            WHERE id = $1
            "#,
            id.as_uuid()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        row.map(Household::try_from).transpose()
    }

    async fn find_personal_by_owner(
        &self,
        owner: &UserId,
    ) -> Result<Option<Household>, HouseholdRepositoryError> {
        let row = sqlx::query_as!(
            HouseholdRow,
            r#"
            SELECT
                id,
                name,
                kind,
                personal_owner_id,
                created_at,
                updated_at
            FROM households
            WHERE kind = 'personal' AND personal_owner_id = $1
            "#,
            owner.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        row.map(Household::try_from).transpose()
    }

    async fn find_for_user(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Household>, HouseholdRepositoryError> {
        let rows = sqlx::query_as!(
            HouseholdRow,
            r#"
            SELECT
                h.id,
                h.name,
                h.kind,
                h.personal_owner_id,
                h.created_at,
                h.updated_at
            FROM households h
            JOIN household_members hm ON hm.household_id = h.id
            WHERE hm.user_id = $1
            "#,
            user_id.as_uuid(),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        rows.into_iter().map(Household::try_from).collect()
    }
}

const HOUSEHOLDS_PERSONAL_OWNER_UNIQUE_INDEX: &str = "households_personal_owner_unique_idx";
const HOUSEHOLDS_PRIMARY_KEY: &str = "households_pkey";

fn map_insert_error(error: sqlx::Error) -> HouseholdRepositoryError {
    if let Some(database_error) = error.as_database_error()
        && database_error.is_unique_violation()
    {
        match database_error.constraint() {
            Some(HOUSEHOLDS_PRIMARY_KEY) => {
                return HouseholdRepositoryError::HouseholdAlreadyExists;
            }
            Some(HOUSEHOLDS_PERSONAL_OWNER_UNIQUE_INDEX) => {
                return HouseholdRepositoryError::PersonalHouseholdAlreadyExists;
            }
            _ => {}
        }
    };

    map_sqlx_error(error).into()
}

struct HouseholdRow {
    id: Uuid,
    name: String,
    kind: String,
    personal_owner_id: Option<Uuid>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<HouseholdRow> for Household {
    type Error = HouseholdRepositoryError;

    fn try_from(value: HouseholdRow) -> Result<Self, Self::Error> {
        let name = HouseholdName::parse(&value.name)
            .map_err(|_| HouseholdRepositoryError::InvalidStoredData)?;

        let kind = HouseholdKind::parse(&value.kind)
            .map_err(|_| HouseholdRepositoryError::InvalidStoredData)?;

        let personal_owner_id = value.personal_owner_id.map(UserId::from_uuid);

        Household::new(
            HouseholdId::from_uuid(value.id),
            name,
            kind,
            personal_owner_id,
            value.created_at,
            value.updated_at,
        )
        .map_err(|_| HouseholdRepositoryError::InvalidStoredData)
    }
}
