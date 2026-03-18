use thiserror::Error;

use crate::Provider;

/// Errors that can occur in adapter operations
#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("authentication failed for {provider:?}: {message}")]
    Auth {
        provider: Provider,
        message: String,
    },

    #[error("resource not found: {resource}")]
    NotFound { resource: String },

    #[error("permission denied: {message}")]
    PermissionDenied { message: String },

    #[error("data residency violation: {message}")]
    ResidencyViolation { message: String },

    #[error("provider error ({provider:?}): {message}")]
    ProviderError {
        provider: Provider,
        message: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("operation not supported by {provider:?}: {operation}")]
    Unsupported {
        provider: Provider,
        operation: String,
    },

    #[error("timeout after {duration_ms}ms")]
    Timeout { duration_ms: u64 },

    #[error("{0}")]
    Other(String),
}
