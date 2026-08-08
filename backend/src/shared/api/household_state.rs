use std::sync::Arc;

use crate::modules::households::application::{
    CreateHouseholdService, ListHouseholdsForUserService,
};

#[derive(Clone)]
pub struct HouseholdsState {
    pub create_household: Arc<CreateHouseholdService>,
    pub list_households_for_user: Arc<ListHouseholdsForUserService>,
}
