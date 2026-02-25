use config::{Config as ConfigLoader, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub model: ModelSettings,
    pub inference: InferenceSettings,
    pub security: SecuritySettings,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerSettings {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModelSettings {
    pub path: PathBuf,
    #[serde(default = "default_context_size")]
    pub context_size: u32,
    #[serde(default = "default_gpu_layers")]
    pub gpu_layers: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InferenceSettings {
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    #[serde(default = "default_repeat_penalty")]
    pub repeat_penalty: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecuritySettings {
    #[serde(default = "default_rate_limit")]
    pub rate_limit_per_minute: u32,
    #[serde(default = "default_max_prompt_length")]
    pub max_prompt_length: usize,
    #[serde(default = "default_max_concurrent_requests")]
    pub max_concurrent_requests: usize,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_timeout() -> u64 {
    300
}

fn default_context_size() -> u32 {
    2048
}

fn default_gpu_layers() -> i32 {
    0
}

fn default_max_tokens() -> usize {
    256
}

fn default_temperature() -> f32 {
    0.7
}

fn default_top_p() -> f32 {
    0.9
}

fn default_repeat_penalty() -> f32 {
    1.1
}

fn default_rate_limit() -> u32 {
    60
}

fn default_max_prompt_length() -> usize {
    4096
}

fn default_max_concurrent_requests() -> usize {
    10
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = ConfigLoader::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("LLM").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                host: default_host(),
                port: default_port(),
                timeout_seconds: default_timeout(),
            },
            model: ModelSettings {
                path: PathBuf::from("./models/model.gguf"),
                context_size: default_context_size(),
                gpu_layers: default_gpu_layers(),
            },
            inference: InferenceSettings {
                max_tokens: default_max_tokens(),
                temperature: default_temperature(),
                top_p: default_top_p(),
                repeat_penalty: default_repeat_penalty(),
            },
            security: SecuritySettings {
                rate_limit_per_minute: default_rate_limit(),
                max_prompt_length: default_max_prompt_length(),
                max_concurrent_requests: default_max_concurrent_requests(),
            },
        }
    }
}
