use crate::modules::accounts::domain::{Email, PasswordHash, UserId};

#[derive(Clone)]
pub struct User {
    id: UserId,
    email: Email,
    password_hash: PasswordHash,
}

impl User {
    pub fn new(id: UserId, email: Email, password_hash: PasswordHash) -> Self {
        Self {
            id,
            email,
            password_hash,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_is_created_with_given_status() {
        let id = UserId::new();
        let email = Email::parse("example.email@email.com").expect("Email should be valid");
        let password_hash =
            PasswordHash::from_encoded("$argon2id$example").expect("Password hash should be valid");

        let user = User::new(id, email.clone(), password_hash.clone());

        assert_eq!(user.id(), id);
        assert_eq!(user.email(), &email);
        assert_eq!(user.password_hash(), &password_hash);
    }
}
