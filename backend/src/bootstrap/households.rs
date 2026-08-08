use std::sync::Arc;

use sqlx::PgPool;

use crate::{
    modules::households::{
        adapters::PostgresHouseholdRepository,
        application::{CreateHouseholdService, ListHouseholdsForUserService},
    },
    shared::api::HouseholdsState,
};

pub(super) fn build_households_state(pool: &PgPool) -> HouseholdsState {
    let household_repository = Arc::new(PostgresHouseholdRepository::new(pool.clone()));

    let create_household_service =
        Arc::new(CreateHouseholdService::new(household_repository.clone()));

    let list_households_for_user_service =
        Arc::new(ListHouseholdsForUserService::new(household_repository));

    HouseholdsState {
        create_household: create_household_service,
        list_households_for_user: list_households_for_user_service,
    }
}
