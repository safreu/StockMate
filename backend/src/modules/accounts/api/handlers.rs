use axum::{Json, extract::State, http::StatusCode};

use crate::{
    modules::accounts::{
        api::{LoginUserRequest, LoginUserResponse, RegisterUserRequest, RegisterUserResponse},
        application::{LoginUserCommand, RegisterUserCommand},
    },
    shared::api::{ApiError, AppState},
};

pub async fn register_user(
    State(state): State<AppState>,
    Json(request): Json<RegisterUserRequest>,
) -> Result<(StatusCode, Json<RegisterUserResponse>), ApiError> {
    let command = RegisterUserCommand {
        email: request.email,
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
    Json(request): Json<LoginUserRequest>,
) -> Result<(StatusCode, Json<LoginUserResponse>), ApiError> {
    let command = LoginUserCommand {
        email: request.email,
        password: request.password,
    };

    let user_id = state.login_user_service.execute(command).await?;

    let response = LoginUserResponse {
        id: user_id.to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}
