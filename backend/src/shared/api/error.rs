use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::modules::accounts::application::{
    AuthenticateSessionError, CreateSessionError, LoginUserError, RegisterUserError,
};

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    body: ApiErrorBody,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

impl ApiError {
    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            body: ApiErrorBody {
                code,
                message: message.into(),
            },
        }
    }

    pub fn invalid_email() -> Self {
        Self::bad_request("invalid_email", "The email address is invalid")
    }

    pub fn invalid_display_name() -> Self {
        Self::bad_request("invalid_display_name", "The display name is invalid")
    }

    pub fn unauthorized(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            body: ApiErrorBody {
                code,
                message: message.into(),
            },
        }
    }

    pub fn authentication_required() -> Self {
        Self::unauthorized("authentication_required", "Authentication is required")
    }

    pub fn invalid_credentials() -> Self {
        Self::unauthorized(
            "invalid_credentials",
            "The supplied credentials are invalid",
        )
    }

    pub fn forbidden(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            body: ApiErrorBody {
                code,
                message: message.into(),
            },
        }
    }

    pub fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            body: ApiErrorBody {
                code,
                message: message.into(),
            },
        }
    }
    pub fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            body: ApiErrorBody {
                code,
                message: message.into(),
            },
        }
    }

    pub fn email_already_exists() -> Self {
        Self::conflict(
            "email_already_exists",
            "A user with this email already exists",
        )
    }

    pub fn unprocessable_entity(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            body: ApiErrorBody {
                code,
                message: message.into(),
            },
        }
    }
    pub fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ApiErrorBody {
                code,
                message: message.into(),
            },
        }
    }

    pub fn internal_error() -> Self {
        Self::internal("internal_error", "An internal error occurred")
    }
}

impl From<RegisterUserError> for ApiError {
    fn from(error: RegisterUserError) -> Self {
        match error {
            RegisterUserError::EmailAlreadyExists => Self::email_already_exists(),
            RegisterUserError::InvalidEmail => Self::invalid_email(),
            RegisterUserError::InvalidDisplayName => Self::invalid_display_name(),
            RegisterUserError::PasswordHashingFailed | RegisterUserError::RepositoryFailed => {
                Self::internal_error()
            }
        }
    }
}

impl From<LoginUserError> for ApiError {
    fn from(error: LoginUserError) -> Self {
        match error {
            LoginUserError::InvalidCredentials => Self::invalid_credentials(),
            LoginUserError::PasswordVerificationError | LoginUserError::RepositoryFailed => {
                Self::internal_error()
            }
        }
    }
}

impl From<CreateSessionError> for ApiError {
    fn from(error: CreateSessionError) -> Self {
        match error {
            CreateSessionError::TokenGenerationFailed
            | CreateSessionError::InvalidSession
            | CreateSessionError::RepositoryFailed => Self::internal_error(),
        }
    }
}

impl From<AuthenticateSessionError> for ApiError {
    fn from(error: AuthenticateSessionError) -> Self {
        match error {
            AuthenticateSessionError::InvalidSession | AuthenticateSessionError::SessionExpired => {
                Self::authentication_required()
            }
            AuthenticateSessionError::RepositoryFailed => Self::internal_error(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiErrorBody {
    pub code: &'static str,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_request_has_correct_status() {
        let error = ApiError::bad_request("test_error", "Request");
        assert_eq!(error.status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn unauthorized_has_correct_status() {
        let error = ApiError::unauthorized("test_error", "unauthorized");
        assert_eq!(error.status, StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn forbidden_has_correct_status() {
        let error = ApiError::forbidden("test_error", "Forbidden");
        assert_eq!(error.status, StatusCode::FORBIDDEN);
    }

    #[test]
    fn not_found_has_correct_status() {
        let error = ApiError::not_found("test_error", "Not found");
        assert_eq!(error.status, StatusCode::NOT_FOUND);
    }

    #[test]
    fn conflict_has_correct_status() {
        let error = ApiError::conflict("test_error", "Conflict");
        assert_eq!(error.status, StatusCode::CONFLICT);
    }

    #[test]
    fn unprocessable_entity_has_correct_status() {
        let error = ApiError::unprocessable_entity("test_error", "Unprocessable entity");
        assert_eq!(error.status, StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn internal_has_correct_status() {
        let error = ApiError::internal("test_error", "Internal");
        assert_eq!(error.status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn error_contains_code_and_message() {
        let error = ApiError::not_found("inventory_not_found", "Inventory item was not found");
        assert_eq!(error.body.code, "inventory_not_found");
        assert_eq!(error.body.message, "Inventory item was not found");
    }

    #[test]
    fn api_error_converts_into_response() {
        let error = ApiError::not_found("inventory_not_found", "Inventory item was not found");
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn authentication_required_has_expected_response() {
        let error = ApiError::authentication_required();
        assert_eq!(error.status, StatusCode::UNAUTHORIZED);
        assert_eq!(error.body.code, "authentication_required");
        assert_eq!(error.body.message, "Authentication is required");
    }

    #[test]
    fn invalid_email_maps_to_bad_request() {
        let error = ApiError::from(RegisterUserError::InvalidEmail);
        assert_eq!(error.status, StatusCode::BAD_REQUEST);
        assert_eq!(error.body.code, "invalid_email")
    }

    #[test]
    fn existing_email_maps_to_conflict() {
        let error = ApiError::from(RegisterUserError::EmailAlreadyExists);
        assert_eq!(error.status, StatusCode::CONFLICT);
        assert_eq!(error.body.code, "email_already_exists")
    }

    #[test]
    fn expired_session_maps_to_authentication_required() {
        let error = ApiError::from(AuthenticateSessionError::SessionExpired);
        assert_eq!(error.status, StatusCode::UNAUTHORIZED);
        assert_eq!(error.body.code, "authentication_required");
    }
}
