use rig::completion::CompletionError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AzureError {
    #[error("Azure API error: {message}")]
    ApiError { message: String, status: u16 },
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureApiError {
    pub error: AzureApiErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureApiErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub details: Vec<serde_json::Value>,
}

impl From<AzureError> for CompletionError {
    fn from(error: AzureError) -> Self {
        match error {
            AzureError::AuthenticationError(msg) => CompletionError::ProviderError(msg),
            AzureError::RequestError(msg) => CompletionError::RequestError(msg.into()),
            AzureError::SerializationError(err) => CompletionError::RequestError(err.to_string().into()),
            AzureError::HttpError(err) => CompletionError::RequestError(err.to_string().into()),
            AzureError::ApiError { message, status } => {
                CompletionError::ProviderError(format!("Azure API error ({}): {}", status, message))
            }
        }
    }
}
