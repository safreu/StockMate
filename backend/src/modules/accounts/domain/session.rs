use crate::modules::accounts::domain::{SessionId, SessionTokenHash, UserId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    id: SessionId,
    user_id: UserId,
    token_hash: SessionTokenHash,
    expires_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl Session {
    pub fn new(
        id: SessionId,
        user_id: UserId,
        token_hash: SessionTokenHash,
        expires_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Result<Self, SessionError> {
        if expires_at <= created_at {
            return Err(SessionError::InvalidExpiration);
        }
        Ok(Self {
            id,
            user_id,
            token_hash,
            expires_at,
            created_at,
        })
    }

    pub fn id(&self) -> SessionId {
        self.id
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn token_hash(&self) -> &SessionTokenHash {
        &self.token_hash
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn is_expired_at(&self, now: DateTime<Utc>) -> bool {
        self.expires_at <= now
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum SessionError {
    #[error("Expiration time must be after creation time")]
    InvalidExpiration,
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};

    use super::*;

    fn test_session(
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    ) -> Result<Session, SessionError> {
        Session::new(
            SessionId::new(),
            UserId::new(),
            SessionTokenHash::from_encoded("this_is_a_hash").expect("Test hash should be valid"),
            expires_at,
            created_at,
        )
    }

    #[test]
    fn session_is_not_expired_before_expiration() {
        let created_at = Utc
            .with_ymd_and_hms(2026, 8, 1, 12, 0, 0)
            .single()
            .expect("Timestamp should be valid");

        let expires_at = created_at + Duration::hours(1);
        let session = test_session(created_at, expires_at).expect("Session should be valid");

        assert!(!session.is_expired_at(created_at))
    }

    #[test]
    fn session_is_expired_at_expiration_time() {
        let created_at = Utc
            .with_ymd_and_hms(2026, 8, 1, 12, 0, 0)
            .single()
            .expect("Timestamp should be valid");

        let expires_at = created_at + Duration::hours(1);
        let session = test_session(created_at, expires_at).expect("Session should be valid!");

        assert!(session.is_expired_at(expires_at))
    }

    #[test]
    fn session_is_expired_after_expiration() {
        let created_at = Utc
            .with_ymd_and_hms(2026, 8, 1, 12, 0, 0)
            .single()
            .expect("Timestamp should be valid");

        let expires_at = created_at + Duration::hours(1);
        let session = test_session(created_at, expires_at).expect("Session should be valid");

        assert!(session.is_expired_at(expires_at + Duration::seconds(1)))
    }

    #[test]
    fn invalid_session_gets_rejected() {
        let created_at = Utc
            .with_ymd_and_hms(2026, 8, 1, 12, 0, 0)
            .single()
            .expect("Timestamp should be valid");

        let expires_at = created_at - Duration::seconds(1);
        let session = test_session(created_at, expires_at);

        assert_eq!(session, Err(SessionError::InvalidExpiration))
    }
}
