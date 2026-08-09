use std::sync::Arc;

use crate::{
    modules::{
        accounts::domain::UserId,
        households::{domain::Household, ports::HouseholdRepository},
    },
    shared::application::InternalError,
};

pub struct ListHouseholdsForUserCommand {
    pub user_id: UserId,
}

pub struct ListHouseholdsForUserService {
    household_repository: Arc<dyn HouseholdRepository>,
}

impl ListHouseholdsForUserService {
    pub fn new(household_repository: Arc<dyn HouseholdRepository>) -> Self {
        Self {
            household_repository,
        }
    }

    pub async fn execute(
        &self,
        command: ListHouseholdsForUserCommand,
    ) -> Result<Vec<Household>, InternalError> {
        self.household_repository
            .find_for_user(&command.user_id)
            .await
            .map_err(|_| InternalError::Failed)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::modules::households::domain::HouseholdKind;
    use crate::test_helpers::{
        FailingHouseholdRepository, build_list_households_service, insert_owned_household,
    };

    #[tokio::test]
    async fn households_for_user_are_returned() {
        let (service, repository) = build_list_households_service();

        let user_id = UserId::new();

        let (personal_household, _) =
            insert_owned_household(&repository, user_id, HouseholdKind::Personal).await;
        let (shared_household, _) =
            insert_owned_household(&repository, user_id, HouseholdKind::Shared).await;

        let result = service
            .execute(ListHouseholdsForUserCommand { user_id })
            .await
            .expect("Household lookup should succeed");

        assert_eq!(result.len(), 2);
        assert!(result.contains(&personal_household));
        assert!(result.contains(&shared_household))
    }

    #[tokio::test]
    async fn repository_failure_returns_repository_failed() {
        let repository = Arc::new(FailingHouseholdRepository);

        let service = ListHouseholdsForUserService::new(repository.clone());

        let user_id = UserId::new();

        let result = service
            .execute(ListHouseholdsForUserCommand { user_id })
            .await;

        assert_eq!(result, Err(InternalError::Failed))
    }
}
