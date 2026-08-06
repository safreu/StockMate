use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::modules::accounts::{
    domain::{Email, User, UserId},
    ports::{UserRepository, UserRepositoryError},
};

pub struct InMemoryUserRepository {
    users: RwLock<HashMap<UserId, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn insert(&self, user: &User) -> Result<(), UserRepositoryError> {
        let mut users = self.users.write().await;

        if users
            .values()
            .any(|existing| existing.email() == user.email())
        {
            return Err(UserRepositoryError::EmailAlreadyExists);
        }

        users.insert(user.id(), user.clone());

        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        let users = self.users.read().await;

        Ok(users.get(id).cloned())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
        let users = self.users.read().await;

        let user = users.values().find(|user| user.email() == email).cloned();

        Ok(user)
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::accounts::domain::PasswordHash;

    use super::*;

    fn create_user(email: &str) -> User {
        User::new(
            UserId::new(),
            Email::parse(email).expect("Email should be valid"),
            PasswordHash::from_encoded("$test$password_hash")
                .expect("Password hash should be valid"),
        )
    }

    #[tokio::test]
    async fn user_can_be_inserted() {
        let user = create_user("this.email@domain.com");

        let repository = InMemoryUserRepository::new();

        let result = repository.insert(&user).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn inserted_user_can_be_found_by_id() {
        let user = create_user("this.email@domain.com");

        let repository = InMemoryUserRepository::new();

        repository
            .insert(&user)
            .await
            .expect("User insertion should succeed");

        let search = repository
            .find_by_id(&user.id())
            .await
            .expect("Repository lookup should succeed");

        let found = search.expect("Inserted user should exist");

        assert_eq!(found.id(), user.id());
        assert_eq!(found.email(), user.email());
    }

    #[tokio::test]
    async fn inserted_user_can_be_found_by_email() {
        let user = create_user("this.email@domain.com");

        let repository = InMemoryUserRepository::new();

        repository
            .insert(&user)
            .await
            .expect("User insertion should succeed");

        let search = repository
            .find_by_email(user.email())
            .await
            .expect("Repository lookup should succeed");

        let found = search.expect("Inserted user should exist");

        assert_eq!(found.id(), user.id());
        assert_eq!(found.email(), user.email());
    }

    #[tokio::test]
    async fn duplicate_email_is_rejected() {
        let user1 = create_user("this.email@domain.com");
        let user2 = create_user("this.email@domain.com");

        let repository = InMemoryUserRepository::new();

        repository
            .insert(&user1)
            .await
            .expect("User insertion should succeed");

        let result = repository.insert(&user2).await;

        assert_eq!(result, Err(UserRepositoryError::EmailAlreadyExists));
    }

    #[tokio::test]
    async fn unknown_user_id_returns_none() {
        let repository = InMemoryUserRepository::new();

        let id = UserId::new();

        let result = repository
            .find_by_id(&id)
            .await
            .expect("Repository lookup should succeed");

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn unknown_user_email_returns_none() {
        let repository = InMemoryUserRepository::new();

        let email = Email::parse("another.email@domain.com").expect("Email should be valid");

        let result = repository
            .find_by_email(&email)
            .await
            .expect("Repository lookup should succeed");

        assert!(result.is_none());
    }
}
