use axum::{Json, extract::State, http::StatusCode};

use crate::{
    modules::{
        accounts::api::CurrentUser,
        households::{
            api::dto::{CreateHouseholdRequest, CreateHouseholdResponse},
            application::CreateHouseholdCommand,
            domain::HouseholdKind,
        },
    },
    shared::api::{ApiError, AppState},
};

pub async fn create_household(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(request): Json<CreateHouseholdRequest>,
) -> Result<(StatusCode, Json<CreateHouseholdResponse>), ApiError> {
    let kind = HouseholdKind::parse(&request.kind).map_err(|_| {
        ApiError::bad_request("invalid_household_kind", "The household kind is invalid")
    })?;

    let command = CreateHouseholdCommand {
        owner_id: current_user.user_id(),
        name: request.name,
        kind,
    };

    let household_id = state
        .create_household_service
        .execute(command)
        .await
        .map_err(ApiError::from)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateHouseholdResponse {
            id: household_id.to_string(),
        }),
    ))
}
