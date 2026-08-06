use std::sync::Arc;

use crate::modules::accounts::application::{
    CreateSessionService, LoginUserService, RegisterUserService,
};

#[derive(Clone)]
pub struct AppState {
    pub register_user_service: Arc<RegisterUserService>,
    pub login_user_service: Arc<LoginUserService>,
    pub create_session_service: Arc<CreateSessionService>,
}
