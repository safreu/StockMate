use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    modules::accounts::{
        domain::{DisplayName, Email, PasswordHash, User, UserId},
        ports::{UserRepository, UserRepositoryError},
    },
    shared::db::map_sqlx_error,
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
        sqlx::query!(
            r#"
            INSERT INTO users (
                id,
                email,
                display_name,
                password_hash
            )
            VALUES ($1, $2, $3, $4)
            "#,
            user.id().into_uuid(),
            user.email().as_str(),
            user.display_name().as_str(),
            user.password_hash().as_str(),
        )
        .execute(&self.pool)
        .await
        .map_err(map_insert_error)?;

        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserRepositoryError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, display_name, password_hash
            FROM users
            WHERE id = $1
            "#,
            id.as_uuid(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        row.map(User::try_from).transpose()
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
        let row = sqlx::query_as!(
            UserRow,
            r#"
            SELECT id, email, display_name, password_hash
            FROM users
            WHERE email = $1
            "#,
            email.as_str(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        row.map(User::try_from).transpose()
    }
}

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    display_name: String,
    password_hash: String,
}

impl TryFrom<UserRow> for User {
    type Error = UserRepositoryError;

    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        let id = UserId::from_uuid(value.id);

        let email =
            Email::parse(&value.email).map_err(|_| UserRepositoryError::InvalidStoredData)?;

        let display_name = DisplayName::parse(&value.display_name)
            .map_err(|_| UserRepositoryError::InvalidStoredData)?;

        let password_hash = PasswordHash::from_encoded(&value.password_hash)
            .map_err(|_| UserRepositoryError::InvalidStoredData)?;

        Ok(User::new(id, email, display_name, password_hash))
    }
}

const USERS_EMAIL_UNIQUE_CONSTRAINT: &str = "users_email_unique";

fn map_insert_error(error: sqlx::Error) -> UserRepositoryError {
    if let Some(database_error) = error.as_database_error()
        && database_error.is_unique_violation()
        && database_error.constraint() == Some(USERS_EMAIL_UNIQUE_CONSTRAINT)
    {
        return UserRepositoryError::EmailAlreadyExists;
    }

    map_sqlx_error(error).into()
}
