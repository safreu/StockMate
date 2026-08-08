use std::sync::Arc;

use crate::modules::households::application::{
    AddHouseholdMemberService, CreateHouseholdService, GetHouseholdService,
    ListHouseholdsForUserService,
};

#[derive(Clone)]
pub struct HouseholdsState {
    pub create_household: Arc<CreateHouseholdService>,
    pub list_households_for_user: Arc<ListHouseholdsForUserService>,
    pub get_household_for_user: Arc<GetHouseholdService>,
    pub add_household_member: Arc<AddHouseholdMemberService>,
}
