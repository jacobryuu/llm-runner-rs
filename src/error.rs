use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("Model loading failed: {0}")]
    ModelLoadError(String),

    #[error("Inference failed: {0}")]
    InferenceError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Request timeout")]
    Timeout,

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, LlmError>;

impl LlmError {
    pub fn status_code(&self) -> u16 {
        match self {
            LlmError::ValidationError(_) => 400,
            LlmError::RateLimitExceeded => 429,
            LlmError::Timeout => 408,
            LlmError::ModelLoadError(_) | LlmError::ConfigError(_) => 503,
            _ => 500,
        }
    }
}
