use axum::{
    Router,
    routing::{delete, post},
};

use crate::{
    modules::inventory::api::handlers::{
        create_category, create_inventory_item, delete_category, list_categories,
    },
    shared::api::AppState,
};

pub fn inventory_routes() -> Router<AppState> {
    Router::new()
        .route("/{household_id}", post(create_inventory_item))
        .route(
            "/{household_id}/categories",
            post(create_category).get(list_categories),
        )
        .route(
            "/{household_id}/categories/{category_id}",
            delete(delete_category),
        )
}
