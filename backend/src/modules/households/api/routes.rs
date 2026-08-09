use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    modules::households::api::handlers::{
        add_household_member, create_household, get_household, list_household_members,
        list_households,
    },
    shared::api::AppState,
};

pub fn households_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_household).get(list_households))
        .route("/{id}", get(get_household))
        .route(
            "/{id}/members",
            post(add_household_member).get(list_household_members),
        )
}
