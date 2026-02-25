pub mod handlers;
pub mod models;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::{
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use std::time::Duration;

use crate::service::LlmService;
use crate::settings::Settings;

pub fn create_router(service: Arc<LlmService>, settings: &Settings) -> Router {
    let timeout = Duration::from_secs(settings.server.timeout_seconds);

    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/v1/chat", post(handlers::chat))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(timeout))
        .with_state(service)
}
