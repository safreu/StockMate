use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

use crate::{
    modules::accounts::{
        api::{LoginUserRequest, LoginUserResponse, RegisterUserRequest, RegisterUserResponse},
        application::{CreateSessionCommand, LoginUserCommand, RegisterUserCommand},
    },
    shared::api::{ApiError, AppState},
};

pub async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<RegisterUserRequest>,
) -> Result<(StatusCode, Json<RegisterUserResponse>), ApiError> {
    let command = RegisterUserCommand {
        email: request.email,
        display_name: request.display_name,
        password: request.password,
    };

    let user_id = state
        .register_user_service
        .execute(command)
        .await
        .map_err(ApiError::from)?;

    let response = RegisterUserResponse {
        id: user_id.to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login_user(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginUserRequest>,
) -> Result<(StatusCode, CookieJar, Json<LoginUserResponse>), ApiError> {
    let command = LoginUserCommand {
        email: request.email,
        password: request.password,
    };

    let user_id = state.login_user_service.execute(command).await?;

    let session = state
        .create_session_service
        .execute(CreateSessionCommand { user_id })
        .await?;

    let cookie = Cookie::build((
        state.session_cookie.name.clone(),
        session.token.into_string(),
    ))
    .path("/")
    .http_only(true)
    .same_site(SameSite::Lax)
    .secure(state.session_cookie.secure)
    .build();

    let jar = jar.add(cookie);

    Ok((
        StatusCode::OK,
        jar,
        Json(LoginUserResponse {
            id: user_id.to_string(),
        }),
    ))
}
