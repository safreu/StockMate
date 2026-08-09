use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, TimeZone, Utc};

use crate::{
    modules::accounts::{
        adapters::{
            Argon2PasswordHasher, InMemorySessionRepository, InMemoryUserRepository,
            Sha256SessionTokenHasher,
        },
        application::{
            AuthenticateSessionService, CreateSessionService, LoginUserService, RegisterUserService,
        },
        domain::{
            DisplayName, Email, PasswordHash, Session, SessionId, SessionToken, SessionTokenHash,
            User, UserId,
        },
        ports::{
            PasswordHasher, PasswordHasherError, SessionTokenGenerator, SessionTokenGeneratorError,
            UserRepository, UserRepositoryError,
        },
    },
    shared::db::PersistenceError,
};

pub fn create_session(token_hash: &str) -> Session {
    let created_at = Utc
        .with_ymd_and_hms(2026, 8, 1, 12, 0, 0)
        .single()
        .expect("Timestamp should be valid");

    let expires_at = created_at + Duration::hours(1);

    Session::new(
        SessionId::new(),
        UserId::new(),
        SessionTokenHash::from_encoded(token_hash).expect("Test token hash should be valid"),
        expires_at,
        created_at,
    )
    .expect("Test session should be valid")
}

pub fn create_user(email: &str) -> User {
    User::new(
        UserId::new(),
        Email::parse(email).expect("Email should be valid"),
        DisplayName::parse("valid name").expect("Display name should be valid"),
        PasswordHash::from_encoded("$test$password_hash").expect("Password hash should be valid"),
    )
}

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

struct FixedSessionTokenGenerator {
    token: String,
}

impl FixedSessionTokenGenerator {
    fn new(token: &str) -> Self {
        Self {
            token: token.to_owned(),
        }
    }
}

impl SessionTokenGenerator for FixedSessionTokenGenerator {
    fn generate(&self) -> Result<SessionToken, SessionTokenGeneratorError> {
        Ok(SessionToken::from_string(self.token.clone())
            .expect("Test session token should be valid"))
    }
}

pub struct FailingSessionTokenGenerator;

impl SessionTokenGenerator for FailingSessionTokenGenerator {
    fn generate(&self) -> Result<SessionToken, SessionTokenGeneratorError> {
        Err(SessionTokenGeneratorError::GenerationFailed)
    }
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

pub struct FailingUserRepository;

#[async_trait]
#[allow(unused)]
impl UserRepository for FailingUserRepository {
    async fn insert(&self, user: &User) -> Result<(), UserRepositoryError> {
        Err(UserRepositoryError::Persistence(PersistenceError::Failed))
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        Err(UserRepositoryError::Persistence(PersistenceError::Failed))
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
        Err(UserRepositoryError::Persistence(PersistenceError::Failed))
    }

    async fn find_by_ids(&self, ids: &[UserId]) -> Result<Vec<User>, UserRepositoryError> {
        Err(UserRepositoryError::Persistence(PersistenceError::Failed))
    }
}

pub struct MissingUserRepository;

#[async_trait]
#[allow(unused)]
impl UserRepository for MissingUserRepository {
    async fn insert(&self, user: &User) -> Result<(), UserRepositoryError> {
        Err(UserRepositoryError::Persistence(PersistenceError::Failed))
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        Err(UserRepositoryError::Persistence(PersistenceError::Failed))
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
        Err(UserRepositoryError::Persistence(PersistenceError::Failed))
    }

    async fn find_by_ids(&self, ids: &[UserId]) -> Result<Vec<User>, UserRepositoryError> {
        Ok(Vec::new())
    }
}
