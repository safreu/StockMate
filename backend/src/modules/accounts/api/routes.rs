use axum::{Router, routing::post};

use crate::{modules::accounts::api::register_user::register_user, shared::api::AppState};

pub fn accounts_router() -> Router<AppState> {
    Router::new().route("/register", post(register_user))
}
