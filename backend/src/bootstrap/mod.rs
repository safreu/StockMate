use crate::{
    bootstrap::{accounts::build_accounts_state, households::build_households_state},
    config::AppConfig,
    shared::{api::AppState, db::create_pool},
};

mod accounts;

mod households;

pub async fn build_app_state(config: &AppConfig) -> Result<AppState, BootstrapError> {
    let pool = create_pool(&config.database).await?;

    let accounts = build_accounts_state(&pool, &config.session);

    let households = build_households_state(&pool);

    Ok(AppState {
        accounts,
        households,
    })
}

#[derive(Debug, thiserror::Error)]
pub enum BootstrapError {
    #[error("Failed to initialize database connection pool")]
    Database(#[from] sqlx::Error),
}
