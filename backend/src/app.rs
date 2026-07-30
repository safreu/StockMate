use crate::{config::AppConfig, shared::db::create_pool};
use axum::{Json, Router, routing::get};
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub database: PgPool,
}

pub struct Application {
    listener: TcpListener,
    router: Router,
}

impl Application {
    pub async fn build(config: AppConfig) -> Result<Self, ApplicationError> {
        let database = create_pool(&config.database).await?;

        let state = AppState { database };

        let router = Router::new()
            .route("/api/v1/health", get(health))
            .with_state(state);

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
