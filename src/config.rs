use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Path to the GGUF model file
    #[arg(short, long)]
    pub model_path: String,

    /// An initial prompt to start the conversation with (optional)
    #[arg(long)]
    pub prompt: Option<String>,

    /// Number of tokens to generate per turn
    #[arg(long, default_value_t = 256)]
    pub max_tokens: usize,

    /// Number of GPU layers to offload
    #[arg(long, default_value_t = 0)]
    pub n_gpu_layers: i32,

    /// Run in server mode (API server)
    #[arg(long, default_value_t = false)]
    pub server_mode: bool,
}
