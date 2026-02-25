use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;
use tracing::info;
use std::pin::Pin;
use futures::Stream;

use crate::api::models::{ChatRequest, ChatResponse, HealthResponse};
use crate::error::{LlmError, Result as LlmResult};
use crate::settings::Settings;

pub struct LlmService {
    settings: Settings,
    semaphore: Arc<Semaphore>,
    start_time: Instant,
}

impl LlmService {
    pub fn new(settings: Settings) -> Self {
        let max_concurrent = settings.security.max_concurrent_requests;
        
        Self {
            settings,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            start_time: Instant::now(),
        }
    }

    pub async fn health_check(&self) -> HealthResponse {
        HealthResponse {
            status: "healthy".to_string(),
            model_loaded: true,
            uptime_seconds: self.start_time.elapsed().as_secs(),
        }
    }

    pub async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let _permit = self.semaphore.acquire().await
            .map_err(|e| LlmError::InternalError(format!("Semaphore error: {}", e)))?;

        info!("Processing chat request");

        // Placeholder for actual inference
        // In production, this would call the inference engine
        let response_text = format!("Response to: {}", request.prompt);
        let tokens = response_text.split_whitespace().count();

        Ok(ChatResponse {
            response: response_text,
            tokens_generated: tokens,
            finish_reason: "stop".to_string(),
        })
    }

    pub async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = std::result::Result<axum::response::sse::Event, std::convert::Infallible>> + Send>>> {
        use crate::api::models::StreamChunk;
        use futures::stream::{self, StreamExt};
        
        let _permit = self.semaphore.acquire().await
            .map_err(|e| LlmError::InternalError(format!("Semaphore error: {}", e)))?;

        info!("Processing streaming chat request");

        // Placeholder response
        let response = self.chat(request).await?;

        let chunks: Vec<String> = response.response
            .chars()
            .collect::<Vec<_>>()
            .chunks(10)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();

        let stream = stream::iter(chunks)
            .map(|chunk| {
                std::result::Result::<_, std::convert::Infallible>::Ok(
                    axum::response::sse::Event::default()
                        .json_data(StreamChunk {
                            delta: chunk,
                            finish_reason: None,
                        })
                        .unwrap()
                )
            })
            .chain(stream::once(async {
                std::result::Result::<_, std::convert::Infallible>::Ok(
                    axum::response::sse::Event::default()
                        .json_data(StreamChunk {
                            delta: String::new(),
                            finish_reason: Some("stop".to_string()),
                        })
                        .unwrap()
                )
            }));

        Ok(Box::pin(stream))
    }
}
