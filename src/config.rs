use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Path to the GGUF model file
    #[arg(short, long)]
    pub model_path: String,

    /// The prompt to start generation with
    #[arg(long)]
    pub prompt: String,

    /// Number of tokens to generate
    #[arg(long, default_value_t = 256)]
    pub max_tokens: usize,

    /// Number of GPU layers to offload
    #[arg(long, default_value_t = 0)]
    pub n_gpu_layers: i32,
}
