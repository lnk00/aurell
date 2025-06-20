use std::env;

use tracing::info;

#[derive(Debug, Clone)]
pub struct Config {
    pub stytch_project_id: String,
    pub stytch_secret: String,
    pub server_port: u16,
    pub server_host: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::dotenv().ok();

        let stytch_project_id = env::var("STYTCH_PROJECT_ID")
            .map_err(|_| ConfigError::Missing("STYTCH_PROJECT_ID".to_string()))?;

        let stytch_secret = env::var("STYTCH_SECRET")
            .map_err(|_| ConfigError::Missing("STYTCH_SECRET".to_string()))?;

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| {
                ConfigError::Invalid("SERVER_PORT must be a valid port number".to_string())
            })?;

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let config = Config {
            stytch_project_id,
            stytch_secret,
            server_port,
            server_host,
        };
        info!("Server starting with config: {:?}", config);

        Ok(config)
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Missing(String),
    Invalid(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Missing(var) => {
                write!(f, "Missing required environment variable: {}", var)
            }
            ConfigError::Invalid(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}
