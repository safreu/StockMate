use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::modules::accounts::{
    domain::{Session, SessionTokenHash},
    ports::{SessionRepository, SessionRepositoryError},
};

pub struct InMemorySessionRepository {
    sessions: RwLock<HashMap<SessionTokenHash, Session>>,
}

impl InMemorySessionRepository {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl SessionRepository for InMemorySessionRepository {
    async fn insert(&self, session: &Session) -> Result<(), SessionRepositoryError> {
        let mut sessions = self.sessions.write().await;

        if sessions.contains_key(session.token_hash()) {
            return Err(SessionRepositoryError::TokenHashAlreadyExists);
        }

        sessions.insert(session.token_hash().clone(), session.clone());

        Ok(())
    }

    async fn find_by_token_hash(
        &self,
        token_hash: &SessionTokenHash,
    ) -> Result<Option<Session>, SessionRepositoryError> {
        let sessions = self.sessions.read().await;

        Ok(sessions.get(token_hash).cloned())
    }

    async fn delete_by_token_hash(
        &self,
        token_hash: &SessionTokenHash,
    ) -> Result<(), SessionRepositoryError> {
        let mut sessions = self.sessions.write().await;

        sessions.remove(token_hash);

        Ok(())
    }
}

impl Default for InMemorySessionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use crate::test_helpers::create_session;

    use super::*;

    #[tokio::test]
    async fn session_can_be_inserted() {
        let repository = InMemorySessionRepository::new();
        let session = create_session("this-is-a-hash");

        assert!(repository.insert(&session).await.is_ok());
    }

    #[tokio::test]
    async fn inserted_session_can_be_found_by_token_hash() {
        let token_hash = "this-is-a-hash";

        let repository = InMemorySessionRepository::new();
        let session = create_session(token_hash);

        repository
            .insert(&session)
            .await
            .expect("Session insertion should succeed");

        let found = repository
            .find_by_token_hash(session.token_hash())
            .await
            .expect("Session lookup should succeed")
            .expect("Inserted session should exist");

        assert_eq!(found.id(), session.id());
        assert_eq!(found.user_id(), session.user_id());
        assert_eq!(found.token_hash(), session.token_hash());
        assert_eq!(found.expires_at(), session.expires_at());
        assert_eq!(found.created_at(), session.created_at());
    }

    #[tokio::test]
    async fn unknown_token_hash_returns_none() {
        let token_hash = "this-is-a-hash";
        let another_token_hash = SessionTokenHash::from_encoded("this-is-another-hash")
            .expect("Another Hash should be valid");

        let repository = InMemorySessionRepository::new();
        let session = create_session(token_hash);

        repository
            .insert(&session)
            .await
            .expect("Session insertion should succeed");

        let found = repository
            .find_by_token_hash(&another_token_hash)
            .await
            .expect("Session lookup should succeed");
        assert!(found.is_none())
    }

    #[tokio::test]
    async fn session_can_be_deleted_by_token_hash() {
        let token_hash = "this-is-a-hash";

        let repository = InMemorySessionRepository::new();
        let session = create_session(token_hash);

        repository
            .insert(&session)
            .await
            .expect("Session insertion should succeed");

        assert!(
            repository
                .delete_by_token_hash(session.token_hash())
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn deleted_session_can_no_longer_be_found() {
        let token_hash = "this-is-a-hash";

        let repository = InMemorySessionRepository::new();
        let session = create_session(token_hash);

        repository
            .insert(&session)
            .await
            .expect("Session insertion should succeed");

        repository
            .delete_by_token_hash(session.token_hash())
            .await
            .expect("Deletion should succeed");

        let result = repository
            .find_by_token_hash(session.token_hash())
            .await
            .expect("Lookup should succeed");

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn duplicate_token_hash_is_rejected() {
        let token_hash = "this-is-a-hash";

        let repository = InMemorySessionRepository::new();
        let session = create_session(token_hash);

        repository
            .insert(&session)
            .await
            .expect("Session insertion should succeed");

        let result = repository.insert(&session).await;

        assert_eq!(result, Err(SessionRepositoryError::TokenHashAlreadyExists))
    }
}
