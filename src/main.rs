mod config;
mod engine;

use crate::config::Config;
use crate::engine::InferenceEngine;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    // CLI引数をパース
    let config = Config::parse();

    // 推論エンジンを初期化
    let mut inference_engine = InferenceEngine::new(&config)?;

    // 推論を実行
    inference_engine.run(&config)?;

    Ok(())
}