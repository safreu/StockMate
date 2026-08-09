use chrono::{Duration, TimeZone, Utc};

use crate::modules::accounts::{
    adapters::InMemoryUserRepository,
    domain::{
        DisplayName, Email, PasswordHash, Session, SessionId, SessionTokenHash, User, UserId,
    },
    ports::UserRepository,
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

pub async fn insert_user(repository: &InMemoryUserRepository, email: &str) -> User {
    let user = create_user(email);

    repository
        .insert(&user)
        .await
        .expect("Test user should be insertable");

    user
}
