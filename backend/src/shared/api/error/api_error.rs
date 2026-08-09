use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    body: ApiErrorBody,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorBody {
    pub code: &'static str,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}

impl ApiError {
    pub fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            body: ApiErrorBody {
                code,
                message: message.into(),
            },
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn code(&self) -> &'static str {
        self.body.code
    }

    pub fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message)
    }

    pub fn unauthorized(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, code, message)
    }

    pub fn forbidden(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, code, message)
    }

    pub fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, code, message)
    }

    pub fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(StatusCode::CONFLICT, code, message)
    }

    pub fn internal_error() -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            "An internal error occurred",
        )
    }
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
}
