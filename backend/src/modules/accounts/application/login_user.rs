use std::sync::Arc;

use crate::{
    modules::accounts::{
        domain::{Email, UserId},
        ports::{PasswordHasher, UserRepository},
    },
    shared::application::InternalError,
};

pub struct LoginUserCommand {
    pub email: String,
    pub password: String,
}

pub struct LoginUserService {
    user_repository: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl LoginUserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
        }
    }

    pub async fn execute(&self, command: LoginUserCommand) -> Result<UserId, LoginUserError> {
        let email = Email::parse(&command.email).map_err(|_| LoginUserError::InvalidCredentials)?;

        let user = self
            .user_repository
            .find_by_email(&email)
            .await
            .map_err(|error| {
                tracing::error!(error = ?error, "Failed to load user during login");
                LoginUserError::Internal(InternalError::Failed)
            })?
            .ok_or(LoginUserError::InvalidCredentials)?;

        let password_hasher = Arc::clone(&self.password_hasher);
        let user_id = user.id();

        let verified = tokio::task::spawn_blocking(move || {
            password_hasher.verify(&command.password, user.password_hash())
        })
        .await
        .map_err(|error| {
            tracing::error!(error = ?error, user_id = %user_id, "Password verification task failed");
            LoginUserError::Internal(InternalError::Failed)
        })?
        .map_err(|error| {
            tracing::error!(error = ?error, user_id = %user_id, "Password verification failed");
            LoginUserError::Internal(InternalError::Failed)
        })?;

        if verified {
            Ok(user_id)
        } else {
            Err(LoginUserError::InvalidCredentials)
        }
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum LoginUserError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error(transparent)]
    Internal(#[from] InternalError),
}

#[cfg(test)]
mod tests {
    use crate::{
        modules::accounts::{
            adapters::{Argon2PasswordHasher, InMemoryUserRepository},
            domain::{DisplayName, PasswordHash, User},
            ports::PasswordHasherError,
        },
        test_helpers::build_login_service,
    };

    use super::*;

    const VALID_EMAIL: &str = "valid.email@test.com";
    const VALID_PASSWORD: &str = "This is a secret password";
    const VALID_NAME: &str = "This is a valid name";

    fn login_command(email: &str, password: &str) -> LoginUserCommand {
        LoginUserCommand {
            email: email.to_owned(),
            password: password.to_owned(),
        }
    }

    fn create_user(
        email: &str,
        display_name: &str,
        password: &str,
        hasher: &Argon2PasswordHasher,
    ) -> User {
        User::new(
            UserId::new(),
            Email::parse(email).expect("Test email should be valid"),
            DisplayName::parse(display_name).expect("Test display name should be valid"),
            hasher
                .hash(password)
                .expect("Test password should be hashable"),
        )
    }

    #[tokio::test]
    async fn correct_credentials_return_user_id() {
        let (service, repository, hasher) = build_login_service();

        let user = create_user(VALID_EMAIL, VALID_NAME, VALID_PASSWORD, &hasher);

        repository
            .insert(&user)
            .await
            .expect("User should be insertable");

        let command = login_command(VALID_EMAIL, VALID_PASSWORD);

        let verified_user = service.execute(command).await.expect("User should login");

        assert_eq!(verified_user, user.id())
    }

    #[tokio::test]
    async fn wrong_email_returns_invalid_credentials() {
        let (service, repository, hasher) = build_login_service();

        let user = create_user(VALID_EMAIL, VALID_NAME, VALID_PASSWORD, &hasher);

        repository
            .insert(&user)
            .await
            .expect("User should be insertable");

        let command = login_command("another.valid@email.com", VALID_PASSWORD);

        let verified_user = service.execute(command).await;

        assert_eq!(verified_user, Err(LoginUserError::InvalidCredentials))
    }

    #[tokio::test]
    async fn wrong_password_returns_invalid_credentials() {
        let (service, repository, hasher) = build_login_service();

        let user = create_user(VALID_EMAIL, VALID_NAME, VALID_PASSWORD, &hasher);

        repository
            .insert(&user)
            .await
            .expect("User should be insertable");

        let command = login_command(VALID_EMAIL, "Another secret password");

        let verified_user = service.execute(command).await;

        assert_eq!(verified_user, Err(LoginUserError::InvalidCredentials))
    }

    #[tokio::test]
    async fn invalid_email_returns_invalid_credentials() {
        let (service, repository, hasher) = build_login_service();

        let user = create_user(VALID_EMAIL, VALID_NAME, VALID_PASSWORD, &hasher);

        repository
            .insert(&user)
            .await
            .expect("User should be insertable");

        let command = login_command("invalid_email.com", VALID_PASSWORD);

        let verified_user = service.execute(command).await;

        assert_eq!(verified_user, Err(LoginUserError::InvalidCredentials))
    }

    #[tokio::test]
    async fn verification_failure_is_reported() {
        let repository = Arc::new(InMemoryUserRepository::new());

        let hasher = Arc::new(Argon2PasswordHasher::new());

        let service = LoginUserService::new(repository.clone(), Arc::new(FailingPasswordHasher));

        let user = create_user(VALID_EMAIL, VALID_NAME, VALID_PASSWORD, &hasher);

        repository
            .insert(&user)
            .await
            .expect("User should be insertable");

        let command = login_command(VALID_EMAIL, VALID_PASSWORD);

        let verified_user = service.execute(command).await;

        assert_eq!(
            verified_user,
            Err(LoginUserError::Internal(InternalError::Failed))
        )
    }

    pub struct FailingPasswordHasher;

    impl PasswordHasher for FailingPasswordHasher {
        #[allow(unused_variables)]
        fn hash(&self, password: &str) -> Result<PasswordHash, PasswordHasherError> {
            Err(PasswordHasherError::HashFailed)
        }
        #[allow(unused_variables)]
        fn verify(&self, password: &str, hash: &PasswordHash) -> Result<bool, PasswordHasherError> {
            Err(PasswordHasherError::VerifyFailed)
        }
    }
}
