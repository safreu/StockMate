use std::sync::Arc;

use crate::{
    config::{AppConfig, SessionCookieConfig},
    modules::accounts::{
        adapters::{
            Argon2PasswordHasher, PostgresSessionRepository, PostgresUserRepository,
            SecureSessionTokenGenerator, Sha256SessionTokenHasher,
        },
        application::{CreateSessionService, LoginUserService, RegisterUserService},
    },
    shared::{api::AppState, db::create_pool},
};

pub async fn build_app_state(config: &AppConfig) -> Result<AppState, BootstrapError> {
    let pool = create_pool(&config.database).await?;

    let user_repository = Arc::new(PostgresUserRepository::new(pool.clone()));

    let session_repository = Arc::new(PostgresSessionRepository::new(pool.clone()));

    let password_hasher = Arc::new(Argon2PasswordHasher::new());

    let session_token_generator = Arc::new(SecureSessionTokenGenerator);

    let session_token_hasher = Arc::new(Sha256SessionTokenHasher);

    let register_user_service = Arc::new(RegisterUserService::new(
        user_repository.clone(),
        password_hasher.clone(),
    ));

    let login_user_service = Arc::new(LoginUserService::new(user_repository, password_hasher));

    let session_lifetime = chrono::Duration::days(config.session.lifetime_days);

    let create_session_service = Arc::new(CreateSessionService::new(
        session_repository,
        session_token_generator,
        session_token_hasher,
        session_lifetime,
    ));

    Ok(AppState {
        register_user_service,
        login_user_service,
        create_session_service,
        session_cookie: SessionCookieConfig {
            name: config.session.cookie_name.clone(),
            secure: config.session.cookie_secure,
        },
    })
}

#[derive(Debug, thiserror::Error)]
pub enum BootstrapError {
    #[error("Failed to initialize database connection pool")]
    Database(#[from] sqlx::Error),
}
