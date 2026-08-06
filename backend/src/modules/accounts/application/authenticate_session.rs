use std::sync::Arc;

use chrono::Utc;

use crate::modules::accounts::{
    domain::{SessionToken, UserId},
    ports::{SessionRepository, SessionTokenHasher},
};

pub struct AuthenticateSessionCommand {
    pub token: SessionToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedUser {
    pub user_id: UserId,
}

pub struct AuthenticateSessionService {
    session_repository: Arc<dyn SessionRepository>,
    token_hasher: Arc<dyn SessionTokenHasher>,
}

impl AuthenticateSessionService {
    pub fn new(
        session_repository: Arc<dyn SessionRepository>,
        token_hasher: Arc<dyn SessionTokenHasher>,
    ) -> Self {
        Self {
            session_repository,
            token_hasher,
        }
    }

    pub async fn execute(
        &self,
        command: AuthenticateSessionCommand,
    ) -> Result<AuthenticatedUser, AuthenticateSessionError> {
        let token_hash = self.token_hasher.hash(&command.token);

        let session = self
            .session_repository
            .find_by_token_hash(&token_hash)
            .await
            .map_err(|error| {
                tracing::error!(error=?error, "Failed to load session during authentication");
                AuthenticateSessionError::RepositoryFailed
            })?
            .ok_or(AuthenticateSessionError::InvalidSession)?;

        if session.is_expired_at(Utc::now()) {
            return Err(AuthenticateSessionError::SessionExpired);
        }

        Ok(AuthenticatedUser {
            user_id: session.user_id(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum AuthenticateSessionError {
    #[error("Session is invalid")]
    InvalidSession,
    #[error("Session has expired")]
    SessionExpired,
    #[error("Session repository failed")]
    RepositoryFailed,
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::modules::accounts::{
        adapters::{InMemorySessionRepository, Sha256SessionTokenHasher},
        domain::{Session, SessionId},
    };

    use super::*;

    fn test_service() -> (
        AuthenticateSessionService,
        Arc<InMemorySessionRepository>,
        Arc<Sha256SessionTokenHasher>,
    ) {
        let repository = Arc::new(InMemorySessionRepository::new());
        let hasher = Arc::new(Sha256SessionTokenHasher);

        let service = AuthenticateSessionService::new(repository.clone(), hasher.clone());

        (service, repository, hasher)
    }

    #[tokio::test]
    async fn valid_session_returns_authenticated_user() {
        let (service, repository, hasher) = test_service();

        let user_id = UserId::new();
        let token = SessionToken::from_string("this-is-a-session-token".to_owned());
        let token_hash = hasher.hash(&token);

        let created_at = Utc::now();
        let expires_at = created_at + Duration::hours(1);

        let session = Session::new(
            SessionId::new(),
            user_id,
            token_hash,
            expires_at,
            created_at,
        )
        .expect("Test session should be valid");

        repository
            .insert(&session)
            .await
            .expect("Test session should be insertable");

        let result = service
            .execute(AuthenticateSessionCommand { token })
            .await
            .expect("Session authentication should succeed");

        assert_eq!(result.user_id, user_id)
    }

    #[tokio::test]
    async fn unknown_token_returns_invalid_session() {
        let (service, _, _) = test_service();

        let token = SessionToken::from_string("unknown-token".to_owned());

        let result = service.execute(AuthenticateSessionCommand { token }).await;

        assert_eq!(result, Err(AuthenticateSessionError::InvalidSession))
    }

    #[tokio::test]
    async fn expired_session_returns_session_expired() {
        let (service, repository, hasher) = test_service();

        let user_id = UserId::new();
        let token = SessionToken::from_string("this-is-a-session-token".to_owned());
        let token_hash = hasher.hash(&token);

        let created_at = Utc::now() - Duration::hours(2);
        let expires_at = Utc::now() - Duration::hours(1);

        let session = Session::new(
            SessionId::new(),
            user_id,
            token_hash,
            expires_at,
            created_at,
        )
        .expect("Test session should be valid");

        repository
            .insert(&session)
            .await
            .expect("Test session should be insertable");

        let result = service.execute(AuthenticateSessionCommand { token }).await;

        assert_eq!(result, Err(AuthenticateSessionError::SessionExpired))
    }
}
