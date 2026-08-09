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

    async fn find_by_ids(&self, ids: &[UserId]) -> Result<Vec<User>, UserRepositoryError> {
        let users = self.users.read().await;

        let result = ids.iter().filter_map(|id| users.get(id).cloned()).collect();

        Ok(result)
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use crate::test_helpers::create_user;

    use super::*;

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

    #[tokio::test]
    async fn existing_users_are_returned() {
        let repository = InMemoryUserRepository::new();

        let first_user = create_user("first@email.com");
        let second_user = create_user("second@email.com");

        repository
            .insert(&first_user)
            .await
            .expect("User insertion should succeed");
        repository
            .insert(&second_user)
            .await
            .expect("User insertion should succeed");

        let ids = vec![first_user.id(), second_user.id()];

        let result = repository
            .find_by_ids(&ids)
            .await
            .expect("User lookup should succeed");

        assert_eq!(result.len(), 2);
        assert!(result.contains(&first_user));
        assert!(result.contains(&second_user));
    }

    #[tokio::test]
    async fn unknown_ids_are_ignored() {
        let repository = InMemoryUserRepository::new();

        let first_user = create_user("first@email.com");
        let second_user = create_user("second@email.com");

        repository
            .insert(&first_user)
            .await
            .expect("User insertion should succeed");

        let ids = vec![first_user.id(), second_user.id()];

        let result = repository
            .find_by_ids(&ids)
            .await
            .expect("User lookup should succeed");

        assert_eq!(result.len(), 1);
        assert!(result.contains(&first_user));
        assert!(!result.contains(&second_user));
    }

    #[tokio::test]
    async fn empty_id_list_returns_empty_vec() {
        let repository = InMemoryUserRepository::new();

        let first_user = create_user("first@email.com");
        let second_user = create_user("second@email.com");

        repository
            .insert(&first_user)
            .await
            .expect("User insertion should succeed");
        repository
            .insert(&second_user)
            .await
            .expect("User insertion should succeed");

        let ids = vec![];

        let result = repository
            .find_by_ids(&ids)
            .await
            .expect("User lookup should succeed");

        assert_eq!(result.len(), 0);
        assert!(!result.contains(&first_user));
        assert!(!result.contains(&second_user));
    }
    #[tokio::test]
    async fn only_requested_users_are_returned() {
        let repository = InMemoryUserRepository::new();

        let first_user = create_user("first@email.com");
        let second_user = create_user("second@email.com");
        let third_user = create_user("third@email.com");

        repository
            .insert(&first_user)
            .await
            .expect("User insertion should succeed");
        repository
            .insert(&second_user)
            .await
            .expect("User insertion should succeed");
        repository
            .insert(&third_user)
            .await
            .expect("User insertion should succeed");

        let ids = vec![first_user.id(), second_user.id()];

        let result = repository
            .find_by_ids(&ids)
            .await
            .expect("User lookup should succeed");

        assert_eq!(result.len(), 2);
        assert!(result.contains(&first_user));
        assert!(result.contains(&second_user));
        assert!(!result.contains(&third_user));
    }
}
