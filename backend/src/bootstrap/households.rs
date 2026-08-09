use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    modules::{
        accounts::adapters::PostgresUserRepository,
        households::{
            adapters::PostgresHouseholdRepository,
            application::{
                AddHouseholdMemberService, CreateHouseholdService, GetHouseholdService,
                ListHouseholdMembersService, ListHouseholdsForUserService,
            },
        },
    },
    shared::api::HouseholdsState,
};

pub(super) fn build_households_state(pool: &PgPool) -> HouseholdsState {
    let household_repository: Arc<PostgresHouseholdRepository> =
        Arc::new(PostgresHouseholdRepository::new(pool.clone()));
    let user_repository = Arc::new(PostgresUserRepository::new(pool.clone()));

    let create_household_service =
        Arc::new(CreateHouseholdService::new(household_repository.clone()));

    let list_households_for_user_service = Arc::new(ListHouseholdsForUserService::new(
        household_repository.clone(),
    ));

    let get_household_for_user_service =
        Arc::new(GetHouseholdService::new(household_repository.clone()));

    let add_household_member_service = Arc::new(AddHouseholdMemberService::new(
        household_repository.clone(),
        user_repository.clone(),
    ));

    let list_household_members_service = Arc::new(ListHouseholdMembersService::new(
        household_repository,
        user_repository,
    ));

    HouseholdsState {
        create_household: create_household_service,
        list_households_for_user: list_households_for_user_service,
        get_household_for_user: get_household_for_user_service,
        add_household_member: add_household_member_service,
        list_household_members: list_household_members_service,
    }
}
