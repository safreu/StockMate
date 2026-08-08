use std::sync::Arc;

use chrono::Utc;

use crate::{
    modules::{
        accounts::domain::UserId,
        households::{
            domain::{
                Household, HouseholdId, HouseholdKind, HouseholdMember, HouseholdName,
                HouseholdRole,
            },
            ports::{HouseholdRepository, HouseholdRepositoryError},
        },
    },
    shared::application::InternalError,
};

pub struct CreateHouseholdCommand {
    pub owner_id: UserId,
    pub name: String,
    pub kind: HouseholdKind,
}

pub struct CreateHouseholdService {
    household_repository: Arc<dyn HouseholdRepository>,
}

impl CreateHouseholdService {
    pub fn new(household_repository: Arc<dyn HouseholdRepository>) -> Self {
        Self {
            household_repository,
        }
    }

    pub async fn execute(
        &self,
        command: CreateHouseholdCommand,
    ) -> Result<HouseholdId, CreateHouseholdError> {
        let name =
            HouseholdName::parse(&command.name).map_err(|_| CreateHouseholdError::InvalidName)?;

        let household_id = HouseholdId::new();
        let now = Utc::now();

        let personal_owner_id = match command.kind {
            HouseholdKind::Personal => Some(command.owner_id),
            HouseholdKind::Shared => None,
        };

        let household = Household::new(
            household_id,
            name,
            command.kind,
            personal_owner_id,
            now,
            now,
        )
        .map_err(|error| {
            tracing::error!(
                error = ?error,
                owner_id = %command.owner_id,
                "failed to construct household"
            );
            CreateHouseholdError::Internal(InternalError::Failed)
        })?;

        let owner = HouseholdMember::new(household_id, command.owner_id, HouseholdRole::Owner, now);

        self.household_repository
            .create_with_owner(&household, &owner)
            .await
            .map_err(|error| match error {
                HouseholdRepositoryError::PersonalHouseholdAlreadyExists => {
                    CreateHouseholdError::PersonalHouseholdAlreadyExists
                }
                _ => CreateHouseholdError::Internal(InternalError::Failed),
            })?;

        Ok(household_id)
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CreateHouseholdError {
    #[error("Invalid household name")]
    InvalidName,
    #[error("Personal household already exists")]
    PersonalHouseholdAlreadyExists,
    #[error(transparent)]
    Internal(#[from] InternalError),
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::{
        modules::households::adapters::InMemoryHouseholdRepository, shared::db::PersistenceError,
    };

    use super::*;

    fn test_service() -> (CreateHouseholdService, Arc<InMemoryHouseholdRepository>) {
        let repository = Arc::new(InMemoryHouseholdRepository::new());

        let service = CreateHouseholdService::new(repository.clone());

        (service, repository)
    }

    fn create_command(owner_id: UserId, kind: HouseholdKind) -> CreateHouseholdCommand {
        CreateHouseholdCommand {
            owner_id,
            name: "this is a valid name".into(),
            kind,
        }
    }

    #[tokio::test]
    async fn personal_household_creation_succeeds() {
        let (service, repository) = test_service();

        let owner_id = UserId::new();

        let command = create_command(owner_id, HouseholdKind::Personal);

        let result = service
            .execute(command)
            .await
            .expect("Household creation should succeed");

        let stored_household = repository
            .find_personal_by_owner(&owner_id)
            .await
            .expect("Household lookup should succeed")
            .expect("Household should exist");

        assert_eq!(result, stored_household.id())
    }

    #[tokio::test]
    async fn shared_household_creation_succeeds() {
        let (service, repository) = test_service();

        let owner_id = UserId::new();

        let command = create_command(owner_id, HouseholdKind::Shared);

        let result = service
            .execute(command)
            .await
            .expect("Household creation should succeed");

        let households = repository
            .find_for_user(&owner_id)
            .await
            .expect("Household lookup should succeed");

        assert_eq!(households.len(), 1);
        assert_eq!(result, households[0].id())
    }

    #[tokio::test]
    async fn invalid_name_returns_invalid_name() {
        let (service, _) = test_service();

        let owner_id = UserId::new();

        let command = CreateHouseholdCommand {
            owner_id,
            name: "       ".into(),
            kind: HouseholdKind::Shared,
        };

        let result = service.execute(command).await;

        assert_eq!(result, Err(CreateHouseholdError::InvalidName))
    }

    #[tokio::test]
    async fn second_personal_household_returns_personal_household_already_exists() {
        let (service, _) = test_service();

        let owner_id = UserId::new();

        let command = create_command(owner_id, HouseholdKind::Personal);

        service
            .execute(command)
            .await
            .expect("Household creation should succeed");

        let command = create_command(owner_id, HouseholdKind::Personal);

        let result = service.execute(command).await;

        assert_eq!(
            result,
            Err(CreateHouseholdError::PersonalHouseholdAlreadyExists)
        )
    }

    #[tokio::test]
    async fn repository_failure_returns_repository_failed() {
        let repository = Arc::new(FailingHouseholdRepository);

        let service = CreateHouseholdService::new(repository.clone());

        let owner_id = UserId::new();

        let command = create_command(owner_id, HouseholdKind::Personal);

        let result = service.execute(command).await;

        assert_eq!(
            result,
            Err(CreateHouseholdError::Internal(InternalError::Failed))
        )
    }

    struct FailingHouseholdRepository;

    #[async_trait]
    #[allow(unused)]
    impl HouseholdRepository for FailingHouseholdRepository {
        async fn create_with_owner(
            &self,
            household: &Household,
            owner: &HouseholdMember,
        ) -> Result<(), HouseholdRepositoryError> {
            Err(HouseholdRepositoryError::Persistence(
                PersistenceError::Failed,
            ))
        }

        async fn find_by_id(
            &self,
            id: &HouseholdId,
        ) -> Result<Option<Household>, HouseholdRepositoryError> {
            unreachable!("Not used in this test")
        }

        async fn find_personal_by_owner(
            &self,
            owner: &UserId,
        ) -> Result<Option<Household>, HouseholdRepositoryError> {
            unreachable!("Not used in this test")
        }

        async fn find_for_user(
            &self,
            user_id: &UserId,
        ) -> Result<Vec<Household>, HouseholdRepositoryError> {
            unreachable!("Not used in this test")
        }
    }
}
