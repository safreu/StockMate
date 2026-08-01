use std::sync::Arc;

use crate::{
    config::AppConfig,
    modules::accounts::{
        adapters::{Argon2PasswordHasher, PostgresUserRepository},
        api::accounts_router,
        application::{LoginUserService, RegisterUserService},
    },
    shared::{api::AppState, db::create_pool},
};
use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

pub struct Application {
    listener: TcpListener,
    router: Router,
}

impl Application {
    pub async fn build(config: AppConfig) -> Result<Self, ApplicationError> {
        let database = create_pool(&config.database).await?;

        let user_repository = Arc::new(PostgresUserRepository::new(database));

        let password_hasher = Arc::new(Argon2PasswordHasher::new());

        let register_user_service = Arc::new(RegisterUserService::new(
            user_repository.clone(),
            password_hasher.clone(),
        ));

        let login_user_service = Arc::new(LoginUserService::new(user_repository, password_hasher));

        let state = AppState {
            register_user_service,
            login_user_service,
        };

        let router = build_router(state);

        let listener = TcpListener::bind(config.server.address).await?;

        Ok(Self { listener, router })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let address = self.listener.local_addr()?;

        tracing::info!(%address, "StockMate backend started");

        axum::serve(self.listener, self.router)
            .with_graceful_shutdown(shutdown_signal())
            .await
    }
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/health", get(health))
        .nest("/api/v1/auth", accounts_router())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(TraceLayer::new_for_http())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .with_state(state)
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "im still alive",
    })
}

async fn shutdown_signal() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => tracing::info!("shutdown signal received"),
        Err(error) => tracing::error!(%error, "could not listen for shutdown signal"),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("could not connect to PostgreSQL")]
    Database(#[from] sqlx::Error),

    #[error("could not bind the HTTP server")]
    IO(#[from] std::io::Error),
}
