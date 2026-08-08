use std::sync::Arc;

use crate::{
    modules::accounts::{
        domain::{DisplayName, Email, User, UserId},
        ports::{PasswordHasher, UserRepository, UserRepositoryError},
    },
    shared::application::InternalError,
};

pub struct RegisterUserCommand {
    pub email: String,
    pub display_name: String,
    pub password: String,
}

pub struct RegisterUserService {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl RegisterUserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
        }
    }

    pub async fn execute(&self, command: RegisterUserCommand) -> Result<UserId, RegisterUserError> {
        let email = Email::parse(&command.email).map_err(|_| RegisterUserError::InvalidEmail)?;

        let display_name = DisplayName::parse(&command.display_name)
            .map_err(|_| RegisterUserError::InvalidDisplayName)?;

        let password_hasher = Arc::clone(&self.password_hasher);
        let password_hash =
            tokio::task::spawn_blocking(move || password_hasher.hash(&command.password))
                .await
                .map_err(|error| {
                    tracing::error!(error = ?error, "password hashing task failed");
                    RegisterUserError::Internal(InternalError::Failed)
                })?
                .map_err(|error| {
                    tracing::error!(error = ?error, "password hashing failed");
                    RegisterUserError::Internal(InternalError::Failed)
                })?;

        let id = UserId::new();

        let user = User::new(id, email, display_name, password_hash);

        self.user_repository
            .insert(&user)
            .await
            .map_err(|error| match error {
                UserRepositoryError::EmailAlreadyExists => {
                    RegisterUserError::EmailAlreadyExists
                },
                other => {
                    tracing::error!(error = ?other, user_id = %user.id(), "failed to persist registered user");
                    RegisterUserError::Internal(InternalError::Failed)
                },
            })?;

        Ok(id)
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum RegisterUserError {
    #[error("Email is invalid")]
    InvalidEmail,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Display name is invalid")]
    InvalidDisplayName,

    #[error(transparent)]
    Internal(#[from] InternalError),
}

#[cfg(test)]
mod tests {

    use crate::{
        modules::accounts::adapters::InMemoryUserRepository,
        test_helpers::{FailingPasswordHasher, build_register_service},
    };

    use super::*;

    const VALID_EMAIL: &str = "valid.email@test.com";
    const VALID_PASSWORD: &str = "This is a secret password";
    const VALID_NAME: &str = "Valid name";

    fn register_command(email: &str, display_name: &str, password: &str) -> RegisterUserCommand {
        RegisterUserCommand {
            email: email.to_owned(),
            display_name: display_name.to_owned(),
            password: password.to_owned(),
        }
    }

    #[tokio::test]
    async fn valid_user_can_be_registered() {
        let (service, repository, _) = build_register_service();

        let id = service
            .execute(register_command(VALID_EMAIL, VALID_NAME, VALID_PASSWORD))
            .await
            .expect("Registration should succeed");

        let stored_user = repository
            .find_by_id(&id)
            .await
            .expect("Repository lookup should succeed")
            .expect("Registered user should exist");

        assert_eq!(id, stored_user.id());
        assert_eq!(VALID_EMAIL, stored_user.email().as_str());
        assert_eq!(VALID_NAME, stored_user.display_name().as_str())
    }

    #[tokio::test]
    async fn password_is_hashed_before_user_is_stored() {
        let (service, repository, hasher) = build_register_service();

        let id = service
            .execute(register_command(VALID_EMAIL, VALID_NAME, VALID_PASSWORD))
            .await
            .expect("Registration should succeed");

        let stored_user = repository
            .find_by_id(&id)
            .await
            .expect("Repository lookup should succeed")
            .expect("Registered user should exist");

        let password_matches = hasher
            .verify(VALID_PASSWORD, stored_user.password_hash())
            .expect("Password verification should succeed");

        assert!(password_matches);
    }

    #[tokio::test]
    async fn invalid_email_is_rejected() {
        let (service, _, _) = build_register_service();

        let result = service
            .execute(register_command(
                "invalid.email-test.com",
                VALID_NAME,
                VALID_PASSWORD,
            ))
            .await;

        assert_eq!(result, Err(RegisterUserError::InvalidEmail));
    }

    #[tokio::test]
    async fn invalid_display_name_is_rejected() {
        let (service, _, _) = build_register_service();

        let result = service
            .execute(register_command(VALID_EMAIL, "         ", VALID_PASSWORD))
            .await;

        assert_eq!(result, Err(RegisterUserError::InvalidDisplayName));
    }

    #[tokio::test]
    async fn duplicate_email_is_rejected() {
        let (service, _, _) = build_register_service();

        service
            .execute(register_command(VALID_EMAIL, VALID_NAME, VALID_PASSWORD))
            .await
            .expect("Registration should succeed");

        let another_user = service
            .execute(register_command(VALID_EMAIL, VALID_NAME, VALID_PASSWORD))
            .await;

        assert_eq!(another_user, Err(RegisterUserError::EmailAlreadyExists));
    }

    #[tokio::test]
    async fn password_hashing_failure_is_reported() {
        let repository = Arc::new(InMemoryUserRepository::new());
        let hasher = Arc::new(FailingPasswordHasher);

        let service = RegisterUserService::new(repository.clone(), hasher.clone());
        let result = service
            .execute(register_command(VALID_EMAIL, VALID_NAME, VALID_PASSWORD))
            .await;

        assert_eq!(
            result,
            Err(RegisterUserError::Internal(InternalError::Failed))
        );
    }
}
