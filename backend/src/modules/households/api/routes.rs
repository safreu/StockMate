use axum::{Router, routing::post};

use crate::{modules::households::api::handlers::create_household, shared::api::AppState};

pub fn households_router() -> Router<AppState> {
    Router::new().route("/", post(create_household))
}
