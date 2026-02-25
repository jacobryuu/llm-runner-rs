use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ChatRequest {
    #[validate(length(min = 1, max = 4096, message = "Prompt must be between 1 and 4096 characters"))]
    pub prompt: String,

    #[validate(range(min = 1, max = 2048, message = "max_tokens must be between 1 and 2048"))]
    pub max_tokens: Option<usize>,

    #[validate(range(min = 0.0, max = 2.0, message = "temperature must be between 0.0 and 2.0"))]
    pub temperature: Option<f32>,

    #[validate(range(min = 0.0, max = 1.0, message = "top_p must be between 0.0 and 1.0"))]
    pub top_p: Option<f32>,

    pub stream: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub tokens_generated: usize,
    pub finish_reason: String,
}

#[derive(Debug, Serialize)]
pub struct StreamChunk {
    pub delta: String,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub model_loaded: bool,
    pub uptime_seconds: u64,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: u16,
}
