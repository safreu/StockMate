use backend::modules::accounts::domain::{
    DisplayName, Email, PasswordHash, Session, SessionId, SessionTokenHash, User, UserId,
};
use chrono::{Duration, TimeZone, Utc};

pub fn test_session(user_id: &UserId, token_hash: &str) -> Session {
    let created_at = Utc
        .with_ymd_and_hms(2026, 8, 1, 12, 0, 0)
        .single()
        .expect("Timestamp should be valid");

    let expires_at = created_at + Duration::hours(1);

    Session::new(
        SessionId::new(),
        *user_id,
        SessionTokenHash::from_encoded(token_hash).expect("Test token hash should be valid"),
        expires_at,
        created_at,
    )
    .expect("Test session should be valid")
}

pub fn test_user(email: &str) -> User {
    User::new(
        UserId::new(),
        Email::parse(email)
            .expect("Test email should be valid"),
        DisplayName::parse("valid name").expect("Test display name should be valid"),
        PasswordHash::from_encoded("$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQ$7Qdih1MuhjZehB6Svms5vcBhkM4A5f7QWwD4iM4R+AE")
            .expect("Test password hash should be valid"),
    )
}
