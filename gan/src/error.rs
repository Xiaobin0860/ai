use anyhow::Error as AnyError;
use serde_json::Error as JsonError;
use thiserror::Error;
use tokio_tungstenite::tungstenite::Error as WsError;
use toml::de::Error as TomlError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] JsonError),
    #[error("TOML error: {0}")]
    Toml(#[from] TomlError),
    #[error("WebSocket error: {0}")]
    Ws(#[from] WsError),
    #[error("{0}")]
    String(String),
    #[error("{0}")]
    Any(#[from] AnyError),
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::String(s)
    }
}

pub type AppResult<T> = Result<T, AppError>;
