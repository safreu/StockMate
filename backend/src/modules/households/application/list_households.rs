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
    use chrono::Utc;

    use super::*;
    use crate::modules::households::domain::HouseholdId;
    use crate::modules::households::domain::{
        HouseholdKind, HouseholdMember, HouseholdName, HouseholdRole,
    };
    use crate::test_helpers::{FailingHouseholdRepository, build_list_households_service};

    fn create_owned_household(
        user_id: UserId,
        kind: HouseholdKind,
    ) -> (Household, HouseholdMember) {
        let household_id = HouseholdId::new();
        let now = Utc::now();

        let owner = HouseholdMember::new(household_id, user_id, HouseholdRole::Owner, now);

        let personal_owner_id = match kind {
            HouseholdKind::Personal => Some(user_id),
            HouseholdKind::Shared => None,
        };

        let household = Household::new(
            household_id,
            HouseholdName::parse("Test household").expect("Test household name should be valid"),
            kind,
            personal_owner_id,
            now,
            now,
        )
        .expect("Test household should be valid");

        (household, owner)
    }

    #[tokio::test]
    async fn households_for_user_are_returned() {
        let (service, repository) = build_list_households_service();

        let user_id = UserId::new();

        let (personal_household, personal_owner) =
            create_owned_household(user_id, HouseholdKind::Personal);
        let (shared_household, shared_owner) =
            create_owned_household(user_id, HouseholdKind::Shared);

        repository
            .create_with_owner(&personal_household, &personal_owner)
            .await
            .expect("Household creation should succeed");
        repository
            .create_with_owner(&shared_household, &shared_owner)
            .await
            .expect("Household creation should succeed");

        let command = ListHouseholdsForUserCommand { user_id };

        let result = service
            .execute(command)
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

        let command = ListHouseholdsForUserCommand { user_id };

        let result = service.execute(command).await;

        assert_eq!(result, Err(InternalError::Failed))
    }
}
