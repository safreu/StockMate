use backend::modules::accounts::{
    adapters::{PostgresSessionRepository, PostgresUserRepository},
    domain::{SessionTokenHash, User},
    ports::{SessionRepository, SessionRepositoryError, UserRepository},
};
use sqlx::PgPool;

use crate::integration::helpers::{test_session, test_user};

async fn insert_test_user(repository: &PostgresUserRepository) -> User {
    let user = test_user("valid@email.com");

    repository
        .insert(&user)
        .await
        .expect("Test user should be insertable");

    user
}

#[sqlx::test]
async fn inserted_session_can_be_found_by_token_hash(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let session_repository = PostgresSessionRepository::new(pool);

    let token_hash = "this-is-a-hashed-token";

    let user = insert_test_user(&user_repository).await;

    let session = test_session(&user.id(), token_hash);

    session_repository
        .insert(&session)
        .await
        .expect("Test session should be insertable");

    let found = session_repository
        .find_by_token_hash(
            &SessionTokenHash::from_encoded(token_hash).expect("Test Token should be valid"),
        )
        .await
        .expect("Session lookup should succeed")
        .expect("Inserted Session should exist");

    assert_eq!(found, session)
}

#[sqlx::test]
async fn unknown_token_hash_returns_none(pool: PgPool) {
    let session_repository = PostgresSessionRepository::new(pool);

    let token_hash = "this-is-a-hashed-token";

    let found = session_repository
        .find_by_token_hash(
            &SessionTokenHash::from_encoded(token_hash).expect("Test Token should be valid"),
        )
        .await
        .expect("Session lookup should succeed");

    assert!(found.is_none())
}

#[sqlx::test]
async fn session_can_be_deleted(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let session_repository = PostgresSessionRepository::new(pool);

    let token_hash = "this-is-a-hashed-token";

    let user = insert_test_user(&user_repository).await;

    let session = test_session(&user.id(), token_hash);

    session_repository
        .insert(&session)
        .await
        .expect("Test session should be insertable");

    let result = session_repository
        .delete_by_token_hash(
            &SessionTokenHash::from_encoded(token_hash).expect("Test Token should be valid"),
        )
        .await;

    let found = session_repository
        .find_by_token_hash(
            &SessionTokenHash::from_encoded(token_hash).expect("Test Token should be valid"),
        )
        .await
        .expect("Session lookup should succeed");

    assert!(result.is_ok());
    assert!(found.is_none())
}

#[sqlx::test]
async fn deleting_unknown_sessions_succeeds(pool: PgPool) {
    let session_repository = PostgresSessionRepository::new(pool);

    let token_hash = "this-is-a-hashed-token";

    let result = session_repository
        .delete_by_token_hash(
            &SessionTokenHash::from_encoded(token_hash).expect("Test Token should be valid"),
        )
        .await;

    assert!(result.is_ok())
}

#[sqlx::test]
async fn duplicate_token_hash_is_rejected(pool: PgPool) {
    let user_repository = PostgresUserRepository::new(pool.clone());
    let session_repository = PostgresSessionRepository::new(pool);

    let token_hash = "this-is-a-hashed-token";

    let user = insert_test_user(&user_repository).await;

    let session = test_session(&user.id(), token_hash);

    let another_session = test_session(&user.id(), token_hash);

    session_repository
        .insert(&session)
        .await
        .expect("Test session should be insertable");

    let result = session_repository.insert(&another_session).await;

    assert_eq!(result, Err(SessionRepositoryError::TokenHashAlreadyExists));
}
