use std::{
    env,
    net::{IpAddr, SocketAddr},
};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub address: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = required_variable("APP_HOST")?
            .parse::<IpAddr>()
            .map_err(ConfigError::InvalidHost)?;

        let port = required_variable("APP_PORT")?
            .parse::<u16>()
            .map_err(ConfigError::InvalidPort)?;

        let database_url = required_variable("DATABASE_URL")?;

        let max_connections = required_variable("DATABASE_MAX_CONNECTIONS")?
            .parse::<u32>()
            .map_err(ConfigError::InvalidMaxConnections)?;

        Ok(Self {
            server: ServerConfig {
                address: SocketAddr::new(host, port),
            },
            database: DatabaseConfig {
                url: database_url,
                max_connections,
            },
        })
    }
}

fn required_variable(name: &'static str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::MissingVariable(name))
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("required environment variable `{0}` is missing")]
    MissingVariable(&'static str),

    #[error("APP_HOST is invalid")]
    InvalidHost(#[source] std::net::AddrParseError),

    #[error("APP_PORT is invalid")]
    InvalidPort(#[source] std::num::ParseIntError),

    #[error("DATABASE_MAX_CONNECTIONS is invalid")]
    InvalidMaxConnections(#[source] std::num::ParseIntError),
}
