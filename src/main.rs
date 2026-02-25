mod config;
mod engine;
mod error;
mod settings;
mod api;
mod service;

use crate::config::Config;
use crate::engine::InferenceEngine;
use crate::error::{LlmError, Result};
use crate::settings::Settings;
use crate::service::LlmService;
use clap::Parser;
use std::io::{self, Write};
use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::parse();

    if config.server_mode {
        run_server(config).await
    } else {
        run_cli(config).await
    }
}

async fn run_server(config: Config) -> Result<()> {
    info!("Starting LLM Runner in server mode");

    let settings = Settings::new()
        .map_err(|e| LlmError::ConfigError(e.to_string()))?;

    let service = Arc::new(LlmService::new(settings.clone()));

    let app = crate::api::create_router(service, &settings);

    let addr = format!("{}:{}", settings.server.host, settings.server.port);
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| LlmError::InternalError(format!("Failed to bind: {}", e)))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| LlmError::InternalError(e.to_string()))?;

    Ok(())
}

async fn run_cli(config: Config) -> Result<()> {
    info!("Starting LLM Runner in CLI mode");

    let backend = LlamaBackend::init()
        .map_err(|e| LlmError::ModelLoadError(e.to_string()))?;
    
    let model_params = LlamaModelParams::default();
    let model = LlamaModel::load_from_file(&backend, &config.model_path, &model_params)
        .map_err(|e| LlmError::ModelLoadError(e.to_string()))?;

    let mut inference_engine = InferenceEngine::new(&model, &backend)?;

    println!("Chat with the model. Type 'exit' or 'quit' to end the session.");

    if let Some(initial_prompt) = &config.prompt {
        println!("> {}", initial_prompt);
        inference_engine.chat(initial_prompt, &config)?;
    }

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;

        let user_input = user_input.trim();

        if user_input.eq_ignore_ascii_case("exit") || user_input.eq_ignore_ascii_case("quit") {
            break;
        }

        if user_input.is_empty() {
            continue;
        }

        inference_engine.chat(user_input, &config)?;
    }

    println!("Session ended.");
    Ok(())
}
