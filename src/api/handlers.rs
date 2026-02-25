use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response, sse::Sse},
    Json,
};
use std::sync::Arc;
use tracing::{info, warn};
use validator::Validate;

use crate::api::models::{ChatRequest, HealthResponse, ErrorResponse};
use crate::error::LlmError;
use crate::service::LlmService;

pub async fn health_check(
    State(service): State<Arc<LlmService>>,
) -> Result<Json<HealthResponse>, ApiError> {
    let health = service.health_check().await;
    Ok(Json(health))
}

pub async fn chat(
    State(service): State<Arc<LlmService>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Response, ApiError> {
    payload.validate().map_err(|e| {
        warn!("Validation failed: {}", e);
        LlmError::ValidationError(e.to_string())
    })?;

    info!("Received chat request, prompt length: {}", payload.prompt.len());

    if payload.stream.unwrap_or(false) {
        let stream = service.chat_stream(payload).await?;
        Ok(Sse::new(stream).into_response())
    } else {
        let response = service.chat(payload).await?;
        info!("Chat completed, tokens generated: {}", response.tokens_generated);
        Ok(Json(response).into_response())
    }
}

pub struct ApiError(LlmError);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = StatusCode::from_u16(self.0.status_code())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let body = Json(ErrorResponse {
            error: self.0.to_string(),
            code: status_code.as_u16(),
        });

        (status_code, body).into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<LlmError>,
{
    fn from(err: E) -> Self {
        ApiError(err.into())
    }
}
