use crate::{modules::households::application::CreateHouseholdError, shared::api::error::ApiError};

impl From<CreateHouseholdError> for ApiError {
    fn from(error: CreateHouseholdError) -> Self {
        match error {
            CreateHouseholdError::Internal(_) => Self::internal_error(),
            CreateHouseholdError::InvalidName => {
                ApiError::bad_request("invalid_household_name", "The household name is invalid")
            }
            CreateHouseholdError::PersonalHouseholdAlreadyExists => ApiError::conflict(
                "personal_household_already_exists",
                "A personal household already exists",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use crate::shared::application::InternalError;

    use super::*;

    #[test]
    fn invalid_household_name_maps_to_bad_request() {
        let error = ApiError::from(CreateHouseholdError::InvalidName);

        assert_eq!(error.status(), StatusCode::BAD_REQUEST);
        assert_eq!(error.code(), "invalid_household_name")
    }

    #[test]
    fn existing_personal_household_maps_to_conflict() {
        let error = ApiError::from(CreateHouseholdError::PersonalHouseholdAlreadyExists);

        assert_eq!(error.status(), StatusCode::CONFLICT);
        assert_eq!(error.code(), "personal_household_already_exists")
    }

    #[test]
    fn internal_household_error_maps_to_internal_server_error() {
        let error = ApiError::from(CreateHouseholdError::Internal(InternalError::Failed));

        assert_eq!(error.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.code(), "internal_error")
    }
}
