use std::sync::Arc;

use crate::{
    config::SessionCookieConfig,
    modules::{
        accounts::application::{
            AuthenticateSessionService, CreateSessionService, LoginUserService, RegisterUserService,
        },
        households::application::{CreateHouseholdService, ListHouseholdsForUserService},
    },
};

#[derive(Clone)]
pub struct AppState {
    pub register_user_service: Arc<RegisterUserService>,
    pub login_user_service: Arc<LoginUserService>,
    pub create_session_service: Arc<CreateSessionService>,
    pub authenticate_session_service: Arc<AuthenticateSessionService>,
    pub session_cookie: SessionCookieConfig,
    pub create_household_service: Arc<CreateHouseholdService>,
    pub list_households_for_user_service: Arc<ListHouseholdsForUserService>,
}
