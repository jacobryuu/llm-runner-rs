use crate::config::Config;
use anyhow::Result;
use std::io::Write;

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel, Special, LlamaChatMessage};
use llama_cpp_2::sampling::LlamaSampler;


pub struct InferenceEngine {
    backend: LlamaBackend,
    model: LlamaModel,
}

impl InferenceEngine {
    /// モデルをロードしてエンジンを初期化
    pub fn new(config: &Config) -> Result<Self> {
        let backend = LlamaBackend::init()?;
        let model_params = LlamaModelParams::default();
        let model =
            LlamaModel::load_from_file(&backend, &config.model_path, &model_params)?;
        Ok(Self { backend, model })
    }

    /// プロンプトから推論を実行し、結果をストリーミング出力
    pub fn run(&mut self, config: &Config) -> Result<()> {
        let ctx_params = LlamaContextParams::default();

        let mut ctx = self.model.new_context(&self.backend, ctx_params)?;

        // Try to apply chat template if available
        let prompt = match self.model.chat_template(None) {
            Ok(template) => {
                let messages = vec![LlamaChatMessage::new("user".to_string(), config.prompt.clone())?];
                match self.model.apply_chat_template(&template, &messages, true) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Warning: Failed to apply chat template: {}", e);
                        config.prompt.clone()
                    }
                }
            },
            Err(_) => config.prompt.clone(),
        };

        let tokens = self
            .model
            .str_to_token(&prompt, AddBos::Always)?;

        let mut batch = LlamaBatch::new(512, 1);
        let last_index: i32 = (tokens.len() - 1) as i32;

        for (i, token) in tokens.iter().enumerate() {
            batch.add(*token, i as i32, &[0], i as i32 == last_index)?;
        }

        ctx.decode(&mut batch)?;

        // Print the prompt only if we are using the raw prompt,
        // or just print the user's input for context.
        // Since the prompt variable might contain system tags, we stick to printing config.prompt
        print!("{}", config.prompt);
        std::io::stdout().flush()?;

        let mut generated_tokens = 0;
        let eos_token = self.model.token_eos();

        // Create a sampler chain with penalties, temperature and top-p
        let mut sampler_chain = LlamaSampler::chain_simple([
            // Penalize repetition to avoid loops and repetitive phrases
            // last_n=64, repeat_penalty=1.1, freq_penalty=0.0, present_penalty=0.0
            LlamaSampler::penalties(64, 1.1, 0.0, 0.0),
            LlamaSampler::temp(0.7), // Slightly lower temperature for more coherent output
            LlamaSampler::top_p(0.9, 1),
            LlamaSampler::dist(1234),
        ]);

        // Inference loop with corrected sampling
        let mut n_curr = batch.n_tokens();
        while generated_tokens < config.max_tokens {
            // Use the sampler chain to sample the next token
            let new_token = sampler_chain.sample(&mut ctx, batch.n_tokens() - 1);

            // Accept the new token into the sampler chain to update internal state (e.g. for repetition penalty)
            sampler_chain.accept(new_token);

            if new_token == eos_token {
                break;
            }

            // The method is `token_to_string`, not `token_to_str`, and it is on the model
            let str_slice = self.model.token_to_str(new_token, Special::Tokenize)?;

            print!("{}", str_slice);
            std::io::stdout().flush()?;

            batch.clear();
            batch.add(new_token, n_curr, &[0], true)?;
            
            n_curr += 1;
            ctx.decode(&mut batch)?;
            generated_tokens += 1;
        }

        println!();
        Ok(())
    }
}