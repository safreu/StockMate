use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    modules::households::api::handlers::{create_household, get_household, list_households},
    shared::api::AppState,
};

pub fn households_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_household).get(list_households))
        .route("/{id}", get(get_household))
}
