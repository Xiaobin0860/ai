use serde_json::Error as JsonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] JsonError),
    #[error("{0}")]
    String(String),
}

impl AppError {
    pub fn from_string(s: String) -> Self {
        AppError::String(s)
    }
}

pub type AppResult<T> = Result<T, AppError>;
