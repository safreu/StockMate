use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row, postgres::PgRow};
use uuid::Uuid;

use crate::modules::accounts::{
    domain::{Session, SessionId, SessionTokenHash, UserId},
    ports::{SessionRepository, SessionRepositoryError},
};

pub struct PostgresSessionRepository {
    pool: PgPool,
}

impl PostgresSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn insert(&self, session: &Session) -> Result<(), SessionRepositoryError> {
        sqlx::query(
            r#"
            INSERT INTO sessions (
                id,
                user_id,
                token_hash,
                expires_at,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(session.id().as_uuid())
        .bind(session.user_id().as_uuid())
        .bind(session.token_hash().as_str())
        .bind(session.expires_at())
        .bind(session.created_at())
        .execute(&self.pool)
        .await
        .map_err(map_insert_error)?;

        Ok(())
    }

    async fn find_by_token_hash(
        &self,
        token_hash: &SessionTokenHash,
    ) -> Result<Option<Session>, SessionRepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at
            FROM sessions
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(map_database_error)?;

        row.map(row_to_session).transpose()
    }

    async fn delete_by_token_hash(
        &self,
        token_hash: &SessionTokenHash,
    ) -> Result<(), SessionRepositoryError> {
        sqlx::query(
            r#"
            DELETE FROM sessions
            WHERE token_hash = $1
            "#,
        )
        .bind(token_hash.as_str())
        .execute(&self.pool)
        .await
        .map_err(map_database_error)?;

        Ok(())
    }
}

fn row_to_session(row: PgRow) -> Result<Session, SessionRepositoryError> {
    let id: Uuid = row
        .try_get("id")
        .map_err(|_| SessionRepositoryError::Database)?;

    let user_id: Uuid = row
        .try_get("user_id")
        .map_err(|_| SessionRepositoryError::Database)?;

    let token_hash: String = row
        .try_get("token_hash")
        .map_err(|_| SessionRepositoryError::Database)?;

    let expires_at: DateTime<Utc> = row
        .try_get("expires_at")
        .map_err(|_| SessionRepositoryError::Database)?;

    let created_at: DateTime<Utc> = row
        .try_get("created_at")
        .map_err(|_| SessionRepositoryError::Database)?;

    Session::new(
        SessionId::from_uuid(id),
        UserId::from_uuid(user_id),
        SessionTokenHash::from_encoded(&token_hash)
            .map_err(|_| SessionRepositoryError::InvalidStoredData)?,
        expires_at,
        created_at,
    )
    .map_err(|_| SessionRepositoryError::InvalidStoredData)
}

const SESSIONS_TOKEN_HASH_UNIQUE_CONSTRAINT: &str = "sessions_token_hash_unique";

fn map_insert_error(error: sqlx::Error) -> SessionRepositoryError {
    if let Some(database_error) = error.as_database_error()
        && database_error.is_unique_violation()
        && database_error.constraint() == Some(SESSIONS_TOKEN_HASH_UNIQUE_CONSTRAINT)
    {
        return SessionRepositoryError::TokenHashAlreadyExists;
    }

    map_database_error(error)
}

fn map_database_error(error: sqlx::Error) -> SessionRepositoryError {
    match error {
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::Io(_) => {
            SessionRepositoryError::Unavailable
        }
        _ => SessionRepositoryError::Database,
    }
}
