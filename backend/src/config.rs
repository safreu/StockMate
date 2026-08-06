use std::{
    env,
    net::{IpAddr, SocketAddr},
};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub session: SessionConfig,
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

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub lifetime_days: i64,
    pub cookie_name: String,
    pub cookie_secure: bool,
}

#[derive(Debug, Clone)]
pub struct SessionCookieConfig {
    pub name: String,
    pub secure: bool,
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

        let session_lifetime_days = required_variable("SESSION_LIFETIME_DAYS")?
            .parse::<i64>()
            .map_err(ConfigError::InvalidSessionLifetime)?;

        if session_lifetime_days <= 0 {
            return Err(ConfigError::NonPositiveSessionLifetime);
        }

        let session_cookie_name = required_variable("SESSION_COOKIE_NAME")?;

        if session_cookie_name.trim().is_empty() {
            return Err(ConfigError::EmptySessionCookieName);
        }

        let session_cookie_secure = required_variable("SESSION_COOKIE_SECURE")?
            .parse::<bool>()
            .map_err(ConfigError::InvalidSessionCookieSecure)?;

        Ok(Self {
            server: ServerConfig {
                address: SocketAddr::new(host, port),
            },
            database: DatabaseConfig {
                url: database_url,
                max_connections,
            },
            session: SessionConfig {
                lifetime_days: session_lifetime_days,
                cookie_name: session_cookie_name,
                cookie_secure: session_cookie_secure,
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

    #[error("SESSION_LIFETIME_DAYS is invalid")]
    InvalidSessionLifetime(#[source] std::num::ParseIntError),

    #[error("SESSION_LIFETIME_DAYS must be greater than zero")]
    NonPositiveSessionLifetime,

    #[error("SESSION_COOKIE_SECURE is invalid")]
    InvalidSessionCookieSecure(#[source] std::str::ParseBoolError),

    #[error("SESSION_COOKIE_NAME must not be empty")]
    EmptySessionCookieName,
}
