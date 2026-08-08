use crate::shared::api::{AccountsState, HouseholdsState};

#[derive(Clone)]
pub struct AppState {
    pub accounts: AccountsState,
    pub households: HouseholdsState,
}
