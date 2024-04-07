use thiserror::Error;

use crate::error::SafeError;

#[derive(Error, Debug)]
pub enum ParseWordError {
    #[error("Word not found in wikitionary: {0}")]
    NotFound(String),
    #[error("Error parsing word {0}")]
    Other(#[from] SafeError),
}

impl From<reqwest::Error> for ParseWordError {
    fn from(e: reqwest::Error) -> Self {
        ParseWordError::Other(e.into())
    }
}

impl From<serde_json::Error> for ParseWordError {
    fn from(e: serde_json::Error) -> Self {
        ParseWordError::Other(e.into())
    }
}
