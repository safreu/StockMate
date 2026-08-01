use std::sync::Arc;

use crate::modules::accounts::application::RegisterUserService;

#[derive(Clone)]
pub struct AppState {
    pub register_user_service: Arc<RegisterUserService>,
}
