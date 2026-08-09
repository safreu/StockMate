use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;

use crate::{
    modules::accounts::{
        application::{AuthenticateSessionCommand, AuthenticateSessionError},
        domain::{SessionToken, UserId},
    },
    shared::api::{ApiError, AppState},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentUser {
    user_id: UserId,
}

impl CurrentUser {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }
}

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar: CookieJar = parts.extract_with_state(state).await.map_err(|rejection| {
            tracing::error!(error = ?rejection, "failed to extract request cookies");
            ApiError::internal_error()
        })?;

        let cookie = jar
            .get(&state.accounts.session_cookie.name)
            .ok_or(AuthenticateSessionError::InvalidSession)
            .map_err(ApiError::from)?;

        let token = SessionToken::from_string(cookie.value().to_owned())
            .map_err(|_| AuthenticateSessionError::InvalidSession)
            .map_err(ApiError::from)?;

        let authenticated = state
            .accounts
            .authenticate_session
            .execute(AuthenticateSessionCommand { token })
            .await
            .map_err(ApiError::from)?;

        Ok(Self::new(authenticated.user_id))
    }
}
