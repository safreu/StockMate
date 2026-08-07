use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    modules::accounts::{
        domain::{Session, SessionId, SessionTokenHash, UserId},
        ports::{SessionRepository, SessionRepositoryError},
    },
    shared::db::{DatabaseErrorKind, classify_sqlx_error},
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
        sqlx::query!(
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
            session.id().into_uuid(),
            session.user_id().into_uuid(),
            session.token_hash().as_str(),
            session.expires_at(),
            session.created_at(),
        )
        .execute(&self.pool)
        .await
        .map_err(map_insert_error)?;

        Ok(())
    }

    async fn find_by_token_hash(
        &self,
        token_hash: &SessionTokenHash,
    ) -> Result<Option<Session>, SessionRepositoryError> {
        let row = sqlx::query_as!(
            SessionRow,
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at
            FROM sessions
            WHERE token_hash = $1
            "#,
            token_hash.as_str(),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_database_error)?;

        row.map(Session::try_from).transpose()
    }

    async fn delete_by_token_hash(
        &self,
        token_hash: &SessionTokenHash,
    ) -> Result<(), SessionRepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE token_hash = $1
            "#,
            token_hash.as_str(),
        )
        .execute(&self.pool)
        .await
        .map_err(map_database_error)?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct SessionRow {
    id: Uuid,
    user_id: Uuid,
    token_hash: String,
    expires_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
}

impl TryFrom<SessionRow> for Session {
    type Error = SessionRepositoryError;

    fn try_from(value: SessionRow) -> Result<Self, Self::Error> {
        Session::new(
            SessionId::from_uuid(value.id),
            UserId::from_uuid(value.user_id),
            SessionTokenHash::from_encoded(&value.token_hash)
                .map_err(|_| SessionRepositoryError::InvalidStoredData)?,
            value.expires_at,
            value.created_at,
        )
        .map_err(|_| SessionRepositoryError::InvalidStoredData)
    }
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
    tracing::error!(
        error = ?error, "session repository database operation failed"
    );
    match classify_sqlx_error(&error) {
        DatabaseErrorKind::Unavailable => SessionRepositoryError::Unavailable,
        DatabaseErrorKind::Other => SessionRepositoryError::Database,
    }
}
