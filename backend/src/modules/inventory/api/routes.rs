use axum::{Router, routing::post};

use crate::{
    modules::inventory::api::handlers::{create_category, create_inventory_item, list_categories},
    shared::api::AppState,
};

pub fn inventory_routes() -> Router<AppState> {
    Router::new()
        .route("/{household_id}", post(create_inventory_item))
        .route(
            "/{household_id}/categories",
            post(create_category).get(list_categories),
        )
}
