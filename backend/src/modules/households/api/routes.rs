use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    modules::households::api::handlers::{create_household, list_households},
    shared::api::AppState,
};

pub fn households_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_household))
        .route("/", get(list_households))
}
