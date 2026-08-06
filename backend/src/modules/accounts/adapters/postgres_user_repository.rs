use async_trait::async_trait;
use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;

use crate::modules::accounts::{
    domain::{Email, PasswordHash, User, UserId},
    ports::{UserRepository, UserRepositoryError},
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn insert(&self, user: &User) -> Result<(), UserRepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO users (
                id,
                email,
                password_hash
            )
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(user.id().as_uuid())
        .bind(user.email().as_str())
        .bind(user.password_hash().as_str())
        .execute(&self.pool)
        .await
        .map_err(map_insert_error)?;

        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT id, email, password_hash
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(map_database_error)?;

        row.map(row_to_user).transpose()
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT id, email, password_hash
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(map_database_error)?;

        row.map(row_to_user).transpose()
    }
}

fn row_to_user(row: PgRow) -> Result<User, UserRepositoryError> {
    let id: Uuid = row
        .try_get("id")
        .map_err(|_| UserRepositoryError::Database)?;

    let email: String = row
        .try_get("email")
        .map_err(|_| UserRepositoryError::Database)?;

    let password_hash: String = row
        .try_get("password_hash")
        .map_err(|_| UserRepositoryError::Database)?;

    let id = UserId::from_uuid(id);

    let email = Email::parse(&email).map_err(|_| UserRepositoryError::InvalidStoredData)?;

    let password_hash = PasswordHash::from_encoded(&password_hash)
        .map_err(|_| UserRepositoryError::InvalidStoredData)?;

    Ok(User::new(id, email, password_hash))
}

const USERS_EMAIL_UNIQUE_CONSTRAINT: &str = "users_email_unique";

fn map_insert_error(error: sqlx::Error) -> UserRepositoryError {
    if let Some(database_error) = error.as_database_error()
        && database_error.is_unique_violation()
        && database_error.constraint() == Some(USERS_EMAIL_UNIQUE_CONSTRAINT)
    {
        return UserRepositoryError::EmailAlreadyExists;
    }

    map_database_error(error)
}

fn map_database_error(error: sqlx::Error) -> UserRepositoryError {
    match error {
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::Io(_) => {
            UserRepositoryError::Unavailable
        }
        _ => UserRepositoryError::Database,
    }
}
