use backend::modules::accounts::{
    adapters::PostgresUserRepository,
    domain::{Email, UserId},
    ports::{UserRepository, UserRepositoryError},
};
use sqlx::PgPool;

use crate::integration::helpers::test_user;

#[sqlx::test]
async fn inserted_user_can_be_found_by_id(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);
    let user = test_user("valid.test@email.com");
    let user_id = user.id();

    repository
        .insert(&user)
        .await
        .expect("Inserting user should succeed");

    let found = repository
        .find_by_id(&user_id)
        .await
        .expect("Finding user should succeed")
        .expect("Inserted user should exist");

    assert_eq!(found.id(), user.id());
    assert_eq!(found.email(), user.email());
    assert_eq!(found.password_hash(), user.password_hash())
}

#[sqlx::test]
async fn inserted_user_can_be_found_by_email(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);
    let user = test_user("valid.test@email.com");
    let user_email = user.email().clone();

    repository
        .insert(&user)
        .await
        .expect("Inserting user should succeed");

    let found = repository
        .find_by_email(&user_email)
        .await
        .expect("Finding user should succeed")
        .expect("Inserted user should exist");

    assert_eq!(found.id(), user.id());
    assert_eq!(found.email(), user.email());
    assert_eq!(found.password_hash(), user.password_hash())
}

#[sqlx::test]
async fn find_by_id_returns_none_when_user_does_not_exist(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);

    let found = repository
        .find_by_id(&UserId::new())
        .await
        .expect("Finding user should succeed");

    assert!(found.is_none())
}

#[sqlx::test]
async fn find_by_email_returns_none_when_user_does_not_exist(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);

    let email = Email::parse("valid.email@test.com").expect("Test email should be valid");

    let found = repository
        .find_by_email(&email)
        .await
        .expect("Finding user should succeed");

    assert!(found.is_none())
}

#[sqlx::test]
async fn duplicate_email_is_mapped_to_email_already_exists(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);
    let user = test_user("valid.test@email.com");
    let another_user = test_user("valid.test@email.com");

    repository
        .insert(&user)
        .await
        .expect("Inserting user should succeed");

    let result = repository.insert(&another_user).await;

    assert_eq!(result, Err(UserRepositoryError::EmailAlreadyExists))
}

#[sqlx::test]
async fn existing_users_are_returned(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);
    let user = test_user("test@email.com");
    let another_user = test_user("another.test@email.com");

    repository
        .insert(&user)
        .await
        .expect("Inserting user should succeed");
    repository
        .insert(&another_user)
        .await
        .expect("Inserting user should succeed");

    let ids = vec![user.id(), another_user.id()];

    let result = repository
        .find_by_ids(&ids)
        .await
        .expect("User lookup should succeed");

    assert_eq!(result.len(), 2);
    assert!(result.contains(&user));
    assert!(result.contains(&another_user));
}

#[sqlx::test]
async fn unknown_ids_are_ignored(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);
    let user = test_user("test@email.com");
    let another_user = test_user("another.test@email.com");

    repository
        .insert(&user)
        .await
        .expect("Inserting user should succeed");

    let ids = vec![user.id(), another_user.id()];

    let result = repository
        .find_by_ids(&ids)
        .await
        .expect("User lookup should succeed");

    assert_eq!(result.len(), 1);
    assert!(result.contains(&user));
    assert!(!result.contains(&another_user));
}

#[sqlx::test]
async fn empty_id_list_returns_empty_vec(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);
    let user = test_user("test@email.com");
    let another_user = test_user("another.test@email.com");

    repository
        .insert(&user)
        .await
        .expect("Inserting user should succeed");
    repository
        .insert(&another_user)
        .await
        .expect("Inserting user should succeed");

    let ids = vec![];

    let result = repository
        .find_by_ids(&ids)
        .await
        .expect("User lookup should succeed");

    assert_eq!(result.len(), 0);
    assert!(!result.contains(&user));
    assert!(!result.contains(&another_user));
}

#[sqlx::test]
async fn only_requested_users_are_returned(pool: PgPool) {
    let repository = PostgresUserRepository::new(pool);
    let user = test_user("test@email.com");
    let another_user = test_user("another.test@email.com");
    let third_user = test_user("third@email.com");

    repository
        .insert(&user)
        .await
        .expect("Inserting user should succeed");
    repository
        .insert(&another_user)
        .await
        .expect("Inserting user should succeed");
    repository
        .insert(&third_user)
        .await
        .expect("Inserting user should succeed");

    let ids = vec![user.id(), another_user.id()];

    let result = repository
        .find_by_ids(&ids)
        .await
        .expect("User lookup should succeed");

    assert_eq!(result.len(), 2);
    assert!(result.contains(&user));
    assert!(result.contains(&another_user));
    assert!(!result.contains(&third_user));
}
