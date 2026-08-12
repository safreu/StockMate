use axum::{Router, routing::post};

use crate::{modules::inventory::api::handlers::create_inventory_item, shared::api::AppState};

pub fn inventory_routes() -> Router<AppState> {
    Router::new().route("/{household_id}", post(create_inventory_item))
}
