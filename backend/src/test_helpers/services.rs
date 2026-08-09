use std::sync::Arc;

use chrono::Duration;

use crate::{
    modules::{
        accounts::{
            adapters::{
                Argon2PasswordHasher, InMemorySessionRepository, InMemoryUserRepository,
                Sha256SessionTokenHasher,
            },
            application::{
                AuthenticateSessionService, CreateSessionService, LoginUserService,
                RegisterUserService,
            },
        },
        households::{
            adapters::InMemoryHouseholdRepository,
            application::{
                AddHouseholdMemberService, CreateHouseholdService, GetHouseholdService,
                ListHouseholdMembersService, ListHouseholdsForUserService,
                RemoveHouseholdMemberService,
            },
        },
    },
    test_helpers::FixedSessionTokenGenerator,
};

pub fn build_auth_service() -> (
    AuthenticateSessionService,
    Arc<InMemorySessionRepository>,
    Arc<Sha256SessionTokenHasher>,
) {
    let repository = Arc::new(InMemorySessionRepository::new());
    let hasher = Arc::new(Sha256SessionTokenHasher);

    let service = AuthenticateSessionService::new(repository.clone(), hasher.clone());

    (service, repository, hasher)
}

pub fn build_register_service() -> (
    RegisterUserService,
    Arc<InMemoryUserRepository>,
    Arc<Argon2PasswordHasher>,
) {
    let repository = Arc::new(InMemoryUserRepository::new());
    let hasher = Arc::new(Argon2PasswordHasher::new());

    let service = RegisterUserService::new(repository.clone(), hasher.clone());

    (service, repository, hasher)
}

pub fn build_create_session_service() -> (
    CreateSessionService,
    Arc<InMemorySessionRepository>,
    Arc<Sha256SessionTokenHasher>,
) {
    let repository = Arc::new(InMemorySessionRepository::new());

    let generator = Arc::new(FixedSessionTokenGenerator::new(
        "this-session-token-is-fixed",
    ));

    let hasher = Arc::new(Sha256SessionTokenHasher::new());

    let service = CreateSessionService::new(
        repository.clone(),
        generator,
        hasher.clone(),
        Duration::hours(1),
    );

    (service, repository, hasher)
}

pub fn build_login_service() -> (
    LoginUserService,
    Arc<InMemoryUserRepository>,
    Arc<Argon2PasswordHasher>,
) {
    let repository = Arc::new(InMemoryUserRepository::new());
    let hasher = Arc::new(Argon2PasswordHasher::new());

    let service = LoginUserService::new(repository.clone(), hasher.clone());

    (service, repository, hasher)
}

pub fn build_create_household_service() -> (CreateHouseholdService, Arc<InMemoryHouseholdRepository>)
{
    let repository = Arc::new(InMemoryHouseholdRepository::new());

    let service = CreateHouseholdService::new(repository.clone());

    (service, repository)
}

pub fn build_list_households_service() -> (
    ListHouseholdsForUserService,
    Arc<InMemoryHouseholdRepository>,
) {
    let repository = Arc::new(InMemoryHouseholdRepository::new());

    let service = ListHouseholdsForUserService::new(repository.clone());

    (service, repository)
}

pub fn build_get_household_service() -> (GetHouseholdService, Arc<InMemoryHouseholdRepository>) {
    let repository = Arc::new(InMemoryHouseholdRepository::new());

    let service = GetHouseholdService::new(repository.clone());

    (service, repository)
}

pub fn build_add_member_service() -> (
    AddHouseholdMemberService,
    Arc<InMemoryHouseholdRepository>,
    Arc<InMemoryUserRepository>,
) {
    let household_repository = Arc::new(InMemoryHouseholdRepository::new());
    let user_repository = Arc::new(InMemoryUserRepository::new());

    let service =
        AddHouseholdMemberService::new(household_repository.clone(), user_repository.clone());

    (service, household_repository, user_repository)
}

pub fn build_list_household_members_service() -> (
    ListHouseholdMembersService,
    Arc<InMemoryHouseholdRepository>,
    Arc<InMemoryUserRepository>,
) {
    let household_repository = Arc::new(InMemoryHouseholdRepository::new());
    let user_repository = Arc::new(InMemoryUserRepository::new());

    let service =
        ListHouseholdMembersService::new(household_repository.clone(), user_repository.clone());

    (service, household_repository, user_repository)
}

pub fn build_remove_household_member_service() -> (
    RemoveHouseholdMemberService,
    Arc<InMemoryHouseholdRepository>,
    Arc<InMemoryUserRepository>,
) {
    let household_repository = Arc::new(InMemoryHouseholdRepository::new());
    let user_repository = Arc::new(InMemoryUserRepository::new());

    let service = RemoveHouseholdMemberService::new(household_repository.clone());

    (service, household_repository, user_repository)
}
