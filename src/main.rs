mod config;
mod engine;

use crate::config::Config;
use crate::engine::InferenceEngine;
use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};

use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;

fn main() -> Result<()> {
    // CLI引数をパース
    let config = Config::parse();

    // BackendとModelの所有権をmain関数で持つ
    let backend = LlamaBackend::init()?;
    let model_params = LlamaModelParams::default();
    let model = LlamaModel::load_from_file(&backend, &config.model_path, &model_params)?;

    // 推論エンジンを初期化（modelとbackendの参照を渡す）
    let mut inference_engine = InferenceEngine::new(&model, &backend)?;

    println!("Chat with the model. Type 'exit' or 'quit' to end the session.");

    // 初期プロンプトがある場合は、それを最初の入力として処理
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
