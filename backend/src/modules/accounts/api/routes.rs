use axum::{Router, routing::post};

use crate::{
    modules::accounts::api::handlers::{login_user, register_user},
    shared::api::AppState,
};

pub fn accounts_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
}
