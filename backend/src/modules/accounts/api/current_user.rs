use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;

use crate::{
    modules::accounts::{
        application::AuthenticateSessionCommand,
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
            ApiError::internal("internal_error", "An internal error occurred")
        })?;

        let cookie = jar
            .get(&state.session_cookie.name)
            .ok_or_else(|| ApiError::unauthorized("unauthorized", "Authentication is required"))?;

        let token = SessionToken::from_string(cookie.value().to_owned());

        let authenticated = state
            .authenticate_session_service
            .execute(AuthenticateSessionCommand { token })
            .await
            .map_err(ApiError::from)?;

        Ok(Self::new(authenticated.user_id))
    }
}
