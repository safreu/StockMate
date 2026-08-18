use crate::shared::api::{AccountsState, HouseholdsState, InventoryItemState};

#[derive(Clone)]
pub struct AppState {
    pub accounts: AccountsState,
    pub households: HouseholdsState,
    pub inventory: InventoryItemState,
}
