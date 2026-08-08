use axum::{Json, extract::State, http::StatusCode};

use crate::{
    modules::{
        accounts::api::CurrentUser,
        households::{
            api::dto::{CreateHouseholdRequest, CreateHouseholdResponse, ListHouseholdResponse},
            application::{CreateHouseholdCommand, ListHouseholdsForUserCommand},
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
        .households
        .create_household
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

pub async fn list_households(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<Vec<ListHouseholdResponse>>, ApiError> {
    let command = ListHouseholdsForUserCommand {
        user_id: current_user.user_id(),
    };

    let households = state
        .households
        .list_households_for_user
        .execute(command)
        .await
        .map_err(ApiError::from)?;

    let response = households
        .into_iter()
        .map(|household| ListHouseholdResponse {
            id: household.id().to_string(),
            name: household.name().as_str().to_owned(),
            kind: household.kind().to_string(),
        })
        .collect();

    Ok(Json(response))
}
